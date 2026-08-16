//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1284/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1284(t12812: f64, t12815: f64, t12818: f64, t12821: f64, t12824: f64, t12829: f64, t12832: f64, t12836: f64, t12839: f64, t12842: f64, t12844: f64, t12846: f64, t12848: f64) -> f64 {
    let t15050 = -t12812 + t12815 + t12818 + t12821 - t12824 - t12829 + t12832 - t12836 + t12839 + t12842 + t12844 - t12846 - t12848;
    t15050
}
