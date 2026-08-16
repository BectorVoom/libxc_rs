//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 207/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk207<F: Float>(t53: F, t60: F, t521: F, t912: F, t50: F, t57: F, t280: F, t814: F, t525: F, t921: F, t62: F, t284: F, t68: F, t183: F, zeta_threshold: F) -> (F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t1395 = t912 * t521;
    let t1398 = t57 * t50;
    let t1402 = piecewise3::<F>(t54, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1395 * t280 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1398 * t814);
    let t1403 = t921 * t525;
    let t1406 = t62 * t50;
    let t1410 = piecewise3::<F>(t61, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1403 * t284 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1406 * t814);
    let t1411 = t1402 + t1410;
    let t1412 = t1411 * t68;
    let t1413 = t1412 * t183;
    (t1411, t1412, t1413)
}
