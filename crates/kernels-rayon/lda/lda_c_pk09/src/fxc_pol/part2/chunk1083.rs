//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1083/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1083(t11820: f64, t11834: f64, t458: f64, t452: f64, t11128: f64, t451: f64, t2103: f64, t2758: f64, t1672: f64, t2835: f64, t2832: f64, t7223: f64) -> (f64, f64, f64, f64, f64) {
    let t11835 = t11820 + t11834;
    let t11836 = t458 * t11835;
    let t11837 = t11836 * t452;
    let t11840 = t451 * t11128;
    let t11843 = t2103 * t2758;
    let t11846 = t2835 * t1672;
    let t11848 = t2832 * t7223;
    (t11837, t11840, t11843, t11846, t11848)
}
