//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1351/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1351<F: Float>(t1: F, t3: F, t604: F, t7337: F, t22843: F, t22844: F, t22847: F, t22850: F, t22853: F, t22857: F, t22859: F, t22860: F, t22861: F, t22862: F, t22863: F, t22868: F) -> F {
    let t23321 = t7337 * t1 * t3 * t604;
    let t23323 = t22843 - t22844 + t22847 - t22850 + t22853 + t22857 - t22859 + t22860 + t22861 - t22862 + t22863 + F::cast_from(0.10821041362364843_f64) * t23321 + t22868;
    t23323
}
