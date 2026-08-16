//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1248/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1248(t20799: f64, t20803: f64, t20805: f64, t20810: f64, t20813: f64, t20816: f64, t20818: f64, t20820: f64, t20822: f64, t20824: f64, t20828: f64, t13707: f64, t20831: f64, t20833: f64, t20834: f64, t20836: f64, t20843: f64, t20845: f64, t20847: f64, t20849: f64, t20852: f64, t20854: f64, t20856: f64) -> (f64, f64) {
    let t22024 = t20799 - t20803 - t20805 + t20810 - t20813 - t20816 + t20818 + t20820 - t20822 - t20824 - t20828;
    let t22025 = -t20831 - t20833 + t20834 - t20836 - t20843 - t20845 - t20847 - t20849 + t13707 + t20852 + t20854 + t20856;
    (t22024, t22025)
}
