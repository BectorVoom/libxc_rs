//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 991/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk991(t2604: f64, t3032: f64, t486: f64, t6851: f64, t1423: f64, t6259: f64, t6255: f64, t1601: f64, t2623: f64, t12555: f64, t6639: f64, t6643: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17628 = t3032 * t2604;
    let t17651 = t486 * t6851;
    let t17666 = t1423 * t6259;
    let t17668 = t1423 * t6255;
    let t17719 = t1601 * t2623;
    let t17734 = t12555 * t6639;
    let t17736 = t12555 * t6643;
    (t17628, t17651, t17666, t17668, t17719, t17734, t17736)
}
