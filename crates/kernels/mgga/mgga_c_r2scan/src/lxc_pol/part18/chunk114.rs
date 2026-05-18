//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 114/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk114<F: Float>(t322: F, t330: F, t333: F, t335: F, t337: F, t339: F, t341: F, t343: F, t352: F, t245: F) -> (F, F) {
    let t323 = t322 <= F::new(0.0);
    let t331 = t322 <= F::new(0.25e1);
    let t354 = piecewise5::<f64>(t323, t330, t331, F::new(1.0) - F::new(0.64e0) * t333 - F::new(0.4352e0) * t335 - F::new(0.1535685604549e1) * t337 + F::new(0.3061560252175e1) * t339 - F::new(0.1915710236206e1) * t341 + F::new(0.516884468372e0) * t343 - F::new(0.51848879792e-1) * t339 * t337, -F::new(0.7e0) * t352);
    let t357 = f64::exp(F::new(1.0) * t245);
    (t354, t357)
}
