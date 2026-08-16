//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 978/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk978(t1397: f64, t2674: f64, t1240: f64, t5624: f64, t93: f64, t1214: f64, t1435: f64, t2481: f64, t1336: f64, t2606: f64, t1625: f64, t2666: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10435 = t2674 * t1397;
    let t10439 = t2674 * t1240;
    let t10441 = t5624 * t93 * t10439;
    let t10443 = t2674 * t1214;
    let t10447 = t2481 * t1435;
    let t10449 = t2606 * t1336;
    let t10450 = t10449 * t1625;
    let t10454 = t2666 * t1336;
    (t10435, t10441, t10443, t10447, t10450, t10454)
}
