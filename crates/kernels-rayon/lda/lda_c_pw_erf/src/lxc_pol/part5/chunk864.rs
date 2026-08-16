//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 864/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk864(t7572: f64, t7579: f64, t7581: f64, t7583: f64, t7587: f64, t7591: f64, t7593: f64, t7595: f64, t7599: f64, t7603: f64, t7607: f64, t7611: f64, t7615: f64, t7617: f64, t7619: f64, t7623: f64, t7627: f64) -> f64 {
    let t8039 = -t7572 + t7579 + t7581 + t7583 + t7587 - t7591 + t7593 - t7595 - t7599 - t7603 + t7607 + t7611 + t7615 + t7617 + t7619 + t7623 + t7627;
    t8039
}
