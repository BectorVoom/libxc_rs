//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1194/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1194(t13299: f64, t33944: f64, t40017: f64, t13287: f64, t31195: f64, t39827: f64, t17912: f64, t31443: f64, t39854: f64, t31797: f64, t31806: f64, t36163: f64, t36176: f64, t36178: f64, t36195: f64, t36199: f64, t36206: f64, t36208: f64, t36211: f64, t36215: f64, t36240: f64, t37918: f64, t37922: f64, t37923: f64) -> f64 {
    let t40450 = t33944 * t13299 * t40017;
    let t40455 = t31195 * t13287 * t39827;
    let t40458 = t31443 * t17912 * t39854;
    let t40460 = t36163 - t36176 - t36178 - t36195 + t36199 - t36206 - t36208 - t36211 - t36215 + 0.94344276868812456204e-2_f64 * t40450 - 0.15724046144802076034e-3_f64 * t31797 - t31806 - t37918 + t37922 - t37923 - 0.80031500487063509015e-2_f64 * t36240 - 0.21437009059034868486e-2_f64 * t40455 - 0.12862205435420921092e-2_f64 * t40458;
    t40460
}
