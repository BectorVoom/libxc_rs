//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 748/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk748(t2972: f64, t7608: f64, t2197: f64, t3241: f64, t205: f64, t2201: f64, t568: f64, t192: f64, t7693: f64, t2140: f64, t2993: f64, t2: f64, t619: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7741 = t2972 * t7608;
    let t7751 = t3241 * t2197;
    let t7752 = t205 * t7751;
    let t7754 = t568 * t2201;
    let t7755 = t205 * t7754;
    let t7757 = t192 * t7693;
    let t7759 = t2993 * t2140;
    let t7762 = t619 * t2;
    (t7741, t7752, t7755, t7757, t7759, t7762)
}
