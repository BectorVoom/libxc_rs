//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1205/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1205(t12812: f64, t12813: f64, t12814: f64, t12817: f64, t12818: f64, t12823: f64, t12824: f64, t12826: f64, t12827: f64, t12829: f64, t12832: f64, t12833: f64, t12834: f64, t12835: f64, t12836: f64, t12839: f64, t12844: f64, t12846: f64, t12849: f64, t12852: f64, t12855: f64, t12857: f64, t9770: f64) -> (f64, f64) {
    let t14392 = t12812 - t12813 - t12814 - t12817 - t12818 - t12823 - t12824 - t12826 + t12827 - t12829 - t12832;
    let t14393 = t12833 + t12834 + t12835 - t9770 + t12836 + t12839 + t12844 - t12846 - t12849 - t12852 + t12855 - t12857;
    (t14392, t14393)
}
