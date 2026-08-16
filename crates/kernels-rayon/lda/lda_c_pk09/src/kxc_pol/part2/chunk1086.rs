//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1086/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1086(t11883: f64, t11897: f64, t467: f64, t452: f64, t1971: f64, t2825: f64, t132: f64, t2824: f64, t93: f64, t2070: f64, t2758: f64, t11129: f64, t477: f64) -> (f64, f64, f64, f64, f64) {
    let t11898 = t11883 + t11897;
    let t11899 = t467 * t11898;
    let t11900 = t11899 * t452;
    let t11903 = t2825 * t1971;
    let t11906 = t132 * t2824;
    let t11907 = t93 * t11906;
    let t11910 = t2070 * t2758;
    let t11913 = t11129 * t477;
    (t11900, t11903, t11907, t11910, t11913)
}
