//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 924/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk924<F: Float>(t7704: F, t902: F, t93: F, t481: F, t1972: F, t2752: F, t132: F, t333: F, t476: F, t11248: F, t2747: F, t747: F, t2743: F, t1995: F, t2938: F, t7340: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11454 = t902 * t7704;
    let t11455 = t93 * t11454;
    let t11456 = t481 * t11455;
    let t11458 = t1972 * t2752;
    let t11460 = t132 * t7704;
    let t11461 = t333 * t11460;
    let t11462 = t476 * t11461;
    let t11464 = t476 * t11248;
    let t11466 = t747 * t2747;
    let t11467 = t481 * t11466;
    let t11469 = t747 * t2743;
    let t11470 = t1995 * t11469;
    let t11472 = t2938 * t7340;
    (t11455, t11456, t11458, t11461, t11462, t11464, t11466, t11467, t11469, t11470, t11472)
}
