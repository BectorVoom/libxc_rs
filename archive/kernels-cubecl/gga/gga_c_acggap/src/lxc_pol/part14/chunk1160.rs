//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1160/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1160<F: Float>(t2297: F, t8901: F, t13364: F, t33944: F, t31254: F, t35476: F, t35480: F, t35485: F, t35497: F, t35503: F, t37570: F, t39985: F, t39987: F, t39990: F, t39995: F, t39999: F, t40003: F, t40005: F, t40009: F, t40011: F, t40015: F) -> (F, F) {
    let t40017 = t2297 * t8901;
    let t40019 = t33944 * t13364 * t40017;
    let t40022 = t35476 + t35480 - t35485 + F::cast_from(0.42874018118069736972e-3_f64) * t39985 + F::cast_from(0.62896184579208304136e-3_f64) * t39987 + F::cast_from(0.62896184579208304136e-3_f64) * t39990 - t37570 + F::cast_from(0.31448092289604152068e-3_f64) * t39995 - F::cast_from(0.94344276868812456205e-2_f64) * t39999 - F::cast_from(0.75475421495049964964e-2_f64) * t40003 + F::cast_from(0.56606566121287473722e-2_f64) * t40005 - F::cast_from(0.31448092289604152068e-3_f64) * t40009 + t35497 - F::cast_from(0.37737710747524982482e-2_f64) * t40011 + F::cast_from(0.31448092289604152068e-3_f64) * t40015 - F::cast_from(0.64311027177104605458e-2_f64) * t40019 - F::cast_from(0.42874018118069736972e-3_f64) * t31254 - t35503;
    (t40017, t40022)
}
