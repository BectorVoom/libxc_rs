//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 952/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk952<F: Float>(t11820: F, t11834: F, t458: F, t452: F, t11128: F, t451: F, t2103: F, t2758: F, t1672: F, t2835: F, t2832: F, t7223: F, t2829: F, t2826: F, t462: F, t2084: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11835 = t11820 + t11834;
    let t11836 = t458 * t11835;
    let t11837 = t11836 * t452;
    let t11840 = t451 * t11128;
    let t11843 = t2103 * t2758;
    let t11846 = t2835 * t1672;
    let t11848 = t2832 * t7223;
    let t11850 = t2829 * t1672;
    let t11852 = t2826 * t1672;
    let t11854 = t462 * t11128;
    let t11857 = t2084 * t2758;
    (t11837, t11840, t11843, t11846, t11848, t11850, t11852, t11854, t11857)
}
