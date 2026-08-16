//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 995/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk995(t1431: f64, t5187: f64, t1441: f64, t1447: f64, t5176: f64, t1989: f64, t3226: f64, t1499: f64, t2090: f64, t3146: f64, t853: f64, t11813: f64, t11815: f64, t11816: f64, t11820: f64, t11823: f64, t11825: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11827 = t5187 * t1431 / 15.0_f64;
    let t11829 = t5187 * t1441 / 9.0_f64;
    let t11830 = t1447 * t5176;
    let t11831 = 4.0_f64 / 15.0_f64 * t11830;
    let t11832 = t3226 * t1989;
    let t11833 = 4.0_f64 / 45.0_f64 * t11832;
    let t11835 = t1499 * t2090 / 10.0_f64;
    let t11837 = t3146 * t853 / 30.0_f64;
    let t11838 = -0.013506172839506173_f64 * t11813 + t11815 - t11816 + t11820 + t11823 + t11825 + t11827 + t11829 + t11831 + t11833 - t11835 - t11837;
    (t11827, t11829, t11831, t11833, t11835, t11837, t11838)
}
