//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 640/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk640<F: Float>(t1214: F, t1248: F, t1250: F, t3720: F, t1222: F, t1235: F, t1238: F, t1252: F, t3663: F, t3667: F, t3671: F, t3674: F, t3679: F, t3684: F, t3686: F, t3689: F, t3694: F, t3701: F, t3705: F, t3708: F, t3711: F, t3714: F, t3718: F) -> (F, F, F, F) {
    let t3721 = t1214 * t1248;
    let t3722 = t3721 * t1250;
    let t3723 = t3720 * t3722;
    let t3726 = -F::cast_from(0.21437009059034868486e-3_f64) * t1235 * t3663 - F::cast_from(0.42874018118069736972e-3_f64) * t3667 * t1238 + F::cast_from(0.42874018118069736972e-3_f64) * t3671 * t3674 - F::cast_from(0.28582678745379824648e-3_f64) * t3679 - t3684 - t3686 / F::cast_from(432.0_f64) - t1222 * t3689 / F::cast_from(288.0_f64) - t1222 * t3694 / F::cast_from(144.0_f64) + t1222 * t3701 / F::cast_from(216.0_f64) + F::cast_from(0.28582678745379824648e-3_f64) * t3705 + F::cast_from(0.42874018118069736972e-3_f64) * t3708 * t1252 + F::cast_from(0.28582678745379824648e-3_f64) * t3711 * t3714 - F::cast_from(0.42874018118069736972e-3_f64) * t3718 * t3723;
    (t3721, t3722, t3723, t3726)
}
