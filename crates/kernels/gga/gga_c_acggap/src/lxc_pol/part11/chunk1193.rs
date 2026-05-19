//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1193/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1193<F: Float>(t1089: F, t4643: F, t598: F, t7533: F, t33953: F, t5275: F, t13287: F, t31195: F, t2299: F, t7630: F, t31812: F, t31816: F, t31822: F, t31825: F, t31832: F, t36289: F, t36293: F, t36294: F, t36296: F, t36300: F, t36303: F, t36306: F, t36308: F, t36310: F, t36314: F) -> (F, F) {
    let t36320 = t598 * t1089 * t4643 * t7533;
    let t36323 = t33953 * t5275;
    let t36325 = t31195 * t13287 * t36323;
    let t36327 = t7630 * t2299;
    let t36329 = -F::cast_from(0.18868855373762491241e-2_f64) * t36289 + t36293 - F::cast_from(0.13976929906490734252e-2_f64) * t36294 + F::cast_from(0.34299214494455789578e-1_f64) * t36296 + t36300 + t36303 - F::cast_from(0.40015750243531754508e-2_f64) * t31812 + F::cast_from(0.20007875121765877254e-2_f64) * t31816 - t36306 / F::new(24.0) - t36308 / F::new(48.0) - t36310 / F::new(48.0) - F::new(0.22921875e-1) * t36314 + F::new(11.0) / F::new(1152.0) * t31822 + F::cast_from(0.34299214494455789578e-2_f64) * t31825 - F::cast_from(0.21437009059034868486e-3_f64) * t36320 - F::cast_from(0.10718504529517434243e-2_f64) * t31832 - F::cast_from(0.21437009059034868486e-2_f64) * t36325 - F::cast_from(0.94344276868812456204e-2_f64) * t36327;
    (t36323, t36329)
}
