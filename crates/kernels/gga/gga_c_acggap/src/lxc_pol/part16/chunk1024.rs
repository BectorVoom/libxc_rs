//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1024/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1024<F: Float>(t7433: F, t9601: F, t1165: F, t26757: F, t604: F, t7413: F, t9583: F, t1181: F, t2068: F, t25706: F, t2297: F, t8901: F, t13364: F, t33944: F, t31254: F, t35476: F, t35480: F, t35485: F, t35497: F, t35503: F, t37570: F, t39985: F, t39987: F, t39990: F, t39995: F, t39999: F, t40003: F) -> (F, F) {
    let t40005 = t7433 * t9601;
    let t40009 = t7413 * t1165 * t604 * t26757;
    let t40011 = t7433 * t9583;
    let t40015 = t2068 * t1181 * t604 * t25706;
    let t40017 = t2297 * t8901;
    let t40019 = t33944 * t13364 * t40017;
    let t40022 = t35476 + t35480 - t35485 + 0.42874018118069736972e-3 * t39985 + 0.62896184579208304136e-3 * t39987 + 0.62896184579208304136e-3 * t39990 - t37570 + 0.31448092289604152068e-3 * t39995 - 0.94344276868812456205e-2 * t39999 - 0.75475421495049964964e-2 * t40003 + 0.56606566121287473722e-2 * t40005 - 0.31448092289604152068e-3 * t40009 + t35497 - 0.37737710747524982482e-2 * t40011 + 0.31448092289604152068e-3 * t40015 - 0.64311027177104605458e-2 * t40019 - 0.42874018118069736972e-3 * t31254 - t35503;
    (t40017, t40022)
}
