//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1193/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1193(t1089: f64, t4643: f64, t598: f64, t7533: f64, t33953: f64, t5275: f64, t13287: f64, t31195: f64, t2299: f64, t7630: f64, t31812: f64, t31816: f64, t31822: f64, t31825: f64, t31832: f64, t36289: f64, t36293: f64, t36294: f64, t36296: f64, t36300: f64, t36303: f64, t36306: f64, t36308: f64, t36310: f64, t36314: f64) -> (f64, f64) {
    let t36320 = t598 * t1089 * t4643 * t7533;
    let t36323 = t33953 * t5275;
    let t36325 = t31195 * t13287 * t36323;
    let t36327 = t7630 * t2299;
    let t36329 = -0.18868855373762491241e-2_f64 * t36289 + t36293 - 0.13976929906490734252e-2_f64 * t36294 + 0.34299214494455789578e-1_f64 * t36296 + t36300 + t36303 - 0.40015750243531754508e-2_f64 * t31812 + 0.20007875121765877254e-2_f64 * t31816 - t36306 / 24.0_f64 - t36308 / 48.0_f64 - t36310 / 48.0_f64 - 0.22921875e-1_f64 * t36314 + 11.0_f64 / 1152.0_f64 * t31822 + 0.34299214494455789578e-2_f64 * t31825 - 0.21437009059034868486e-3_f64 * t36320 - 0.10718504529517434243e-2_f64 * t31832 - 0.21437009059034868486e-2_f64 * t36325 - 0.94344276868812456204e-2_f64 * t36327;
    (t36323, t36329)
}
