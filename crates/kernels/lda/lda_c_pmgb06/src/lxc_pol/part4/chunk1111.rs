//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1111/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1111<F: Float>(t1447: F, t5451: F, t5454: F, t5458: F, t5499: F, t1920: F, t3226: F, t5464: F, t5467: F, t5471: F, t5474: F, t1912: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13836 = t1447 * t5451;
    let t13838 = t1447 * t5454;
    let t13840 = t5499 * t5458;
    let t13842 = t3226 * t1920;
    let t13844 = t1447 * t5464;
    let t13846 = t1447 * t5467;
    let t13848 = t1447 * t5471;
    let t13850 = t5499 * t5474;
    let t13883 = t3226 * t1912;
    (t13836, t13838, t13840, t13842, t13844, t13846, t13848, t13850, t13883)
}
