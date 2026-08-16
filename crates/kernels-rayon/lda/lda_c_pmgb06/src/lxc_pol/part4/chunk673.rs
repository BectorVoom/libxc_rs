//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 673/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk673(t3703: f64, t3738: f64, t3741: f64, t696: f64, t1089: f64, t273: f64, t698: f64, t968: f64, t971: f64, t1830: f64, t2060: f64, t3680: f64, t3683: f64, t3685: f64, t3690: f64, t3692: f64, t3694: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3742 = t3738 * t3703 * t3741;
    let t3744 = 1025.4018858216407_f64 * t696 * t3742;
    let t3745 = t1089 * t273;
    let t3746 = t3745 * t698;
    let t3748 = t971 * t968;
    let t3758 = -3.4523333333333333_f64 * t3680 + 2.3015555555555554_f64 * t3683 - 2.6851481481481483_f64 * t3685 - 0.9393222222222222_f64 * t1830 + 0.073355_f64 * t3690 - 0.14671_f64 * t3692 - 0.17116166666666666_f64 * t3694 - 0.36793333333333333_f64 * t2060;
    (t3742, t3744, t3745, t3746, t3748, t3758)
}
