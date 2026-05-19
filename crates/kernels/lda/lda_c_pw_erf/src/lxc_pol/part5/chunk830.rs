//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 830/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk830<F: Float>(t3893: F, t7624: F, t519: F, t7267: F, t7579: F, t7581: F, t7583: F, t7587: F, t7591: F, t7593: F, t7595: F, t7599: F, t7603: F, t7607: F, t7611: F, t7615: F, t7617: F, t7619: F, t7623: F) -> (F, F, F) {
    let t7625 = t3893 * t7624;
    let t7627 = F::new(32.0) / F::new(81.0) * t519 * t7625;
    let t7629 = t7579 + t7581 + t7583 + t7587 - t7591 + t7593 - t7595 - t7599 - t7603 + t7607 + t7611 + t7615 + t7617 + t7619 + t7623 + t7627 + F::cast_from(0.3246312408709453_f64) * t7267;
    (t7625, t7627, t7629)
}
