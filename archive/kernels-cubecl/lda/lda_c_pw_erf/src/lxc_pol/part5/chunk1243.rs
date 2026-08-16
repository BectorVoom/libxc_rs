//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1243/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1243<F: Float>(t2153: F, t7007: F, t1466: F, t2065: F, t571: F, t6193: F, t1318: F, t3899: F, t7584: F, t2146: F, t6705: F, t6702: F) -> (F, F, F, F, F) {
    let t22334 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t7007 * t2153;
    let t22338 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t571 * t1466 * t6193 * t2065;
    let t22340 = t1318 * t3899 * t7584;
    let t22341 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t22340;
    let t22342 = t2146 * t6705;
    let t22343 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t22342;
    let t22344 = t2146 * t6702;
    (t22334, t22338, t22341, t22343, t22344)
}
