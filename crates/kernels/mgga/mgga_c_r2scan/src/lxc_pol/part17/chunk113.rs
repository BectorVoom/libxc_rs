//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 113/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk113<F: Float>(t322: F, t330: F, t333: F, t335: F, t337: F, t339: F, t341: F, t343: F, t352: F, t245: F) -> (F, F) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t354 = piecewise5(t323, t330, t331, 1.0 - 0.64e0 * t333 - 0.4352e0 * t335 - 0.1535685604549e1 * t337 + 0.3061560252175e1 * t339 - 0.1915710236206e1 * t341 + 0.516884468372e0 * t343 - 0.51848879792e-1 * t339 * t337, -0.7e0 * t352);
    let t357 = f64::exp(1.0 * t245);
    (t354, t357)
}
