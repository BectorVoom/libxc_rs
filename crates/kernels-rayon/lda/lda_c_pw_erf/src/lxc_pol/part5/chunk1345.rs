//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1345/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1345(t13919: f64, t13925: f64, t22362: f64, t22363: f64, t22367: f64, t22369: f64, t22372: f64, t22374: f64, t22376: f64, t22378: f64, t22380: f64, t22382: f64, t22384: f64) -> f64 {
    let t23307 = -0.2885611029963958_f64 * t13919 - t13925 - t22362 - t22363 - t22367 + t22369 + t22372 - t22374 + t22376 - t22378 - t22380 - t22382 - t22384;
    t23307
}
