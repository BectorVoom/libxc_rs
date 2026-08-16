//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1160/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1160(t2297: f64, t8901: f64, t13364: f64, t33944: f64, t31254: f64, t35476: f64, t35480: f64, t35485: f64, t35497: f64, t35503: f64, t37570: f64, t39985: f64, t39987: f64, t39990: f64, t39995: f64, t39999: f64, t40003: f64, t40005: f64, t40009: f64, t40011: f64, t40015: f64) -> (f64, f64) {
    let t40017 = t2297 * t8901;
    let t40019 = t33944 * t13364 * t40017;
    let t40022 = t35476 + t35480 - t35485 + 0.42874018118069736972e-3_f64 * t39985 + 0.62896184579208304136e-3_f64 * t39987 + 0.62896184579208304136e-3_f64 * t39990 - t37570 + 0.31448092289604152068e-3_f64 * t39995 - 0.94344276868812456205e-2_f64 * t39999 - 0.75475421495049964964e-2_f64 * t40003 + 0.56606566121287473722e-2_f64 * t40005 - 0.31448092289604152068e-3_f64 * t40009 + t35497 - 0.37737710747524982482e-2_f64 * t40011 + 0.31448092289604152068e-3_f64 * t40015 - 0.64311027177104605458e-2_f64 * t40019 - 0.42874018118069736972e-3_f64 * t31254 - t35503;
    (t40017, t40022)
}
