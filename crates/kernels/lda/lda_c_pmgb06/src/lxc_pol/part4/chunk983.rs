//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 983/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk983<F: Float>(t1447: F, t5487: F, t1423: F, t5483: F, t1555: F, t1848: F, t3155: F, t831: F, t1395: F, t1531: F, t177: F, t2911: F, t2918: F, t1593: F, t13007: F, t5091: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13249 = t1447 * t5487;
    let t13251 = t1423 * t5483;
    let t13291 = t1848 * t1555;
    let t13294 = t831 * t3155;
    let t13296 = t1395 * t1531;
    let t13300 = t177 * t2911;
    let t13304 = t177 * t2918;
    let t13308 = t1593 * t1531;
    let t13312 = t13007 * t5091;
    (t13249, t13251, t13291, t13294, t13296, t13300, t13304, t13308, t13312)
}
