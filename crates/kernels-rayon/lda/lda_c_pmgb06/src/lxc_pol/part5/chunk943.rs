//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 943/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk943(t13708: f64, t1710: f64, t801: f64, t446: f64, t3259: f64, t813: f64, t1969: f64, t3213: f64, t1886: f64, t607: f64, t1966: f64, t3031: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13709 = 2.0_f64 / 135.0_f64 * t13708;
    let t13712 = t801 * t1710;
    let t13713 = t13712 * t446;
    let t13714 = 2.0_f64 / 135.0_f64 * t13713;
    let t13715 = t3259 * t813;
    let t13719 = t3213 * t1969;
    let t13720 = 2.0_f64 / 45.0_f64 * t13719;
    let t13726 = t1886 * t607;
    let t13788 = t1966 * t3031;
    (t13709, t13712, t13714, t13715, t13720, t13726, t13788)
}
