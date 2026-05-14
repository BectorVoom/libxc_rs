//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 759/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk759<F: Float>(t4049: F, t7612: F, t571: F, t2171: F, t2550: F, t2554: F, t523: F, t7360: F, t522: F, t519: F, t3894: F, t7354: F, t3893: F, t7267: F, t7579: F, t7581: F, t7583: F, t7587: F, t7591: F, t7593: F, t7595: F, t7599: F, t7603: F, t7607: F, t7611: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7613 = t4049 * t7612;
    let t7615 = 32.0 / 81.0 * t571 * t7613;
    let t7617 = 4.0 / 15.0 * t2171 * t2550;
    let t7619 = 4.0 / 9.0 * t2171 * t2554;
    let t7620 = t523 * t7360;
    let t7621 = t522 * t7620;
    let t7623 = 4.0 / 45.0 * t519 * t7621;
    let t7624 = t3894 * t7354;
    let t7625 = t3893 * t7624;
    let t7627 = 32.0 / 81.0 * t519 * t7625;
    let t7629 = t7579 + t7581 + t7583 + t7587 - t7591 + t7593 - t7595 - t7599 - t7603 + t7607 + t7611 + t7615 + t7617 + t7619 + t7623 + t7627 + 0.3246312408709453 * t7267;
    (t7613, t7615, t7617, t7619, t7620, t7621, t7623, t7624, t7625, t7627, t7629)
}
