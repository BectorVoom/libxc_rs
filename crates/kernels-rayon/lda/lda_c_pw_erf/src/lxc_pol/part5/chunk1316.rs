//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1316/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1316(t12197: f64, t12310: f64, t12357: f64, t21257: f64, t21261: f64, t21262: f64, t21263: f64, t21264: f64, t21265: f64, t21266: f64, t21267: f64, t21269: f64, t21271: f64) -> f64 {
    let t23227 = -t21257 - t21261 + t21262 + t21263 - t21264 + t12197 + t12310 + t21265 - t21266 + t21267 - t21269 + t12357 + t21271;
    t23227
}
