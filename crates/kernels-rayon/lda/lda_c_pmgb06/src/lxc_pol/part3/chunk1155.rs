//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1155/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1155(t1966: f64, t3031: f64, t1629: f64, t439: f64, t477: f64, t822: f64, t1916: f64, t3198: f64, t13762: f64, t13764: f64, t13767: f64, t13769: f64, t13771: f64, t13774: f64, t13776: f64, t13781: f64, t13783: f64, t13787: f64) -> (f64, f64, f64) {
    let t13788 = t1966 * t3031;
    let t13793 = 3.0_f64 / 5.0_f64 * t439 * t13788 * t822 * t1629 * t477;
    let t13795 = 2.0_f64 / 15.0_f64 * t3198 * t1916;
    let t13796 = t13762 + t13764 + t13767 - t13769 - t13771 + t13774 - t13776 + t13781 - t13783 + t13787 - t13793 - t13795;
    (t13793, t13795, t13796)
}
