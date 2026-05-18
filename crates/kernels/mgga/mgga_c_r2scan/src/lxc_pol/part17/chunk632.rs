//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 632/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk632<F: Float>(t322: F, t2438: F, t3518: F, t352: F, t3520: F, t3548: F, t3549: F, t3552: F, t3556: F, t855: F, t3431: F, t3441: F, t3444: F) -> (F, F, F, F) {
    let t323 = t322 <= F::new(0.0);
    let t331 = t322 <= F::new(0.25e1);
    let t3560 = piecewise5::<f64>(t323, t3518 + t3520, t331, t3548, -F::new(0.21e1) * t3549 * t2438 - F::new(0.105e1) * t855 * t3552 * t352 - F::new(0.1575e1) * t3556 * t2438);
    let t3564 = F::new(0.30487649791575028312e-3) * t3431;
    let t3565 = F::new(0.43368970657079495308e-4) * t3441;
    let t3566 = F::new(0.81300399444200075499e-3) * t3444;
    (t3560, t3564, t3565, t3566)
}
