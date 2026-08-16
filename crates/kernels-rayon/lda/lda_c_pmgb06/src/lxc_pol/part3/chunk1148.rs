//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1148/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1148(t132: f64, t1547: f64, t2042: f64, t1963: f64, t3213: f64, t1423: f64, t4772: f64, t1710: f64, t801: f64, t446: f64, t3259: f64, t813: f64) -> (f64, f64, f64, f64, f64) {
    let t13706 = t132 * t1547 * t2042;
    let t13707 = t13706 / 45.0_f64;
    let t13708 = t3213 * t1963;
    let t13709 = 2.0_f64 / 135.0_f64 * t13708;
    let t13710 = t1423 * t4772;
    let t13711 = 4.0_f64 / 45.0_f64 * t13710;
    let t13712 = t801 * t1710;
    let t13713 = t13712 * t446;
    let t13714 = 2.0_f64 / 135.0_f64 * t13713;
    let t13715 = t3259 * t813;
    (t13707, t13709, t13711, t13714, t13715)
}
