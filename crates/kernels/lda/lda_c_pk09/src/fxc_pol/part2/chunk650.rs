//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 650/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk650<F: Float>(t429: F, t7299: F, t2035: F, t2042: F, t1819: F, t7286: F, t337: F, t450: F, t2000: F, t471: F, t6196: F, t463: F, t7066: F, t6319: F, t6325: F, t6464: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7300 = t7299 * t429;
    let t7301 = t2035 * t7300;
    let t7302 = t7301 * t2042;
    let t7304 = t1819 * t7286;
    let t7307 = t450 * t337;
    let t7308 = t7307 * t429;
    let t7309 = t1819 * t7308;
    let t7310 = t7309 * t2042;
    let t7312 = t471 * t2000;
    let t7313 = t7312 * t6196;
    let t7321 = t463 * t7066;
    let t7324 = 0.10237773105191754 * t6319;
    let t7325 = 0.06825182070127836 * t6325;
    let t7326 = 0.02275060690042612 * t6464;
    (t7300, t7302, t7304, t7308, t7310, t7312, t7313, t7321, t7324, t7325, t7326)
}
