//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 468/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk468<F: Float>(t2758: F, t462: F, t451: F, t2075: F, t2077: F, t2079: F, t2081: F, t2733: F, t2736: F, t2803: F, t2807: F, t458: F, t452: F, t2094: F, t2096: F, t2098: F, t2100: F) -> (F, F, F, F, F, F) {
    let t2835 = t462 * t2758;
    let t2838 = t451 * t2758;
    let t2845 = t2075 - 0.7661514025603425 * t2803 + t2077 + 0.7661514025603425 * t2807 + t2079 - 0.15282509383508946 * t2733 + t2081 + 0.15282509383508946 * t2736;
    let t2846 = t458 * t2845;
    let t2847 = t2846 * t452;
    let t2854 = t2094 - 1.4770435158815312 * t2803 + t2096 + 1.4770435158815312 * t2807 + t2098 - 0.2946275542389858 * t2733 + t2100 + 0.2946275542389858 * t2736;
    (t2835, t2838, t2845, t2846, t2847, t2854)
}
