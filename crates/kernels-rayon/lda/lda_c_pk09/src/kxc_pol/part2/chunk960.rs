//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 960/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk960(t1435: f64, t2580: f64, t318: f64, t9819: f64, t1625: f64, t1336: f64, t2579: f64, t2578: f64, t305: f64, t1632: f64, t304: f64, t10059: f64, t328: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10140 = t2580 * t1435;
    let t10142 = t318 * t9819;
    let t10143 = t10142 * t1625;
    let t10145 = t2579 * t1336;
    let t10146 = t10145 * t1625;
    let t10150 = t2578 * t305;
    let t10151 = t1632 * t10150;
    let t10154 = t304 * t9819;
    let t10155 = t10154 * t1625;
    let t10162 = t328 * t10059;
    (t10140, t10143, t10146, t10151, t10155, t10162)
}
