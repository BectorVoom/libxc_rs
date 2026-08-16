//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1107/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1107(t161: f64, t4802: f64, t489: f64, t132: f64, t1547: f64, t2042: f64, t1963: f64, t3213: f64, t1423: f64, t4772: f64, t1710: f64, t801: f64) -> (f64, f64, f64, f64, f64) {
    let t13686 = t161 * t489 * t4802;
    let t13706 = t132 * t1547 * t2042;
    let t13708 = t3213 * t1963;
    let t13710 = t1423 * t4772;
    let t13712 = t801 * t1710;
    (t13686, t13706, t13708, t13710, t13712)
}
