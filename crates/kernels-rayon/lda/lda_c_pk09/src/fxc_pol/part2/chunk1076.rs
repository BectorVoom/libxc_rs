//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1076/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1076(t471: f64, t476: f64, t2006: f64, t2811: f64, t7292: f64, t2016: f64, t309: f64, t454: f64, t2812: f64, t7300: f64, t2042: f64, t1905: f64, t7704: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11711 = t471 * t476;
    let t11714 = t2811 * t2006;
    let t11715 = t11714 * t7292;
    let t11717 = t309 * t454 * t2016;
    let t11720 = t2812 * t7300;
    let t11721 = t11720 * t2042;
    let t11723 = t2812 * t7292;
    let t11733 = t309 * t1905 * t7704;
    (t11711, t11715, t11717, t11721, t11723, t11733)
}
