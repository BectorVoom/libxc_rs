//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 491/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk491<F: Float>(t53: F, t5133: F, t5279: F, t1411: F, t941: F, t3985: F, t521: F, t50: F, t912: F, t280: F, t814: F, t1395: F, t1398: F, t154: F, t57: F, t913: F, t916: F, zeta_threshold: F) -> (F, F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t5280 = t5133 + t5279;
    let t5321 = t941 * t1411;
    let t5324 = t3985 * t521;
    let t5327 = t912 * t50;
    let t5328 = t814 * t280;
    let t5338 = piecewise3::<f64>(t54, F::new(0.0), -F::new(8.0) / F::new(27.0) * t5324 * t913 + F::new(16.0) / F::new(9.0) * t5327 * t5328 + F::new(4.0) / F::new(9.0) * t1395 * t916 + F::new(8.0) / F::new(3.0) * t57 * t814 - F::new(8.0) * t1398 * t154);
    (t5280, t5321, t5328, t5338)
}
