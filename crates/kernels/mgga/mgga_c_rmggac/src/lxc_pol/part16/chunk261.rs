//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 261/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk261<F: Float>(t53: F, t60: F, t521: F, t912: F, t50: F, t57: F, t280: F, t814: F, t525: F, t921: F, t62: F, t284: F, zeta_threshold: F) -> (F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t1395 = t912 * t521;
    let t1398 = t57 * t50;
    let t1402 = piecewise3::<f64>(t54, F::new(0.0), F::new(4.0) / F::new(9.0) * t1395 * t280 + F::new(8.0) / F::new(3.0) * t1398 * t814);
    let t1403 = t921 * t525;
    let t1406 = t62 * t50;
    let t1410 = piecewise3::<f64>(t61, F::new(0.0), F::new(4.0) / F::new(9.0) * t1403 * t284 - F::new(8.0) / F::new(3.0) * t1406 * t814);
    let t1411 = t1402 + t1410;
    (t1395, t1403, t1411)
}
