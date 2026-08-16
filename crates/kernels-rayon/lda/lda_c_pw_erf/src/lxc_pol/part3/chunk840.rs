//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 840/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk840(t5049: f64, t5051: f64, t5053: f64, t5055: f64, t5057: f64, t5059: f64, t5061: f64, t5067: f64, t5071: f64, t5131: f64, t5133: f64, t5135: f64, t5140: f64, t5145: f64, t5150: f64, t5154: f64, t5159: f64) -> f64 {
    let t5863 = -t5049 + t5051 - t5053 - t5055 - t5057 + t5059 + t5061 + t5067 + t5071 - t5131 - t5133 + t5135 - t5140 - t5145 + t5150 - t5154 - t5159;
    t5863
}
