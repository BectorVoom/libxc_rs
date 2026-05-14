//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1134/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1134<F: Float>(t1245: F, t2497: F, t3402: F, t519: F, t940: F, t2171: F, t5261: F, t16634: F, t16637: F, t16640: F, t16643: F, t16647: F, t16649: F, t16651: F, t16653: F, t16656: F, t16660: F, t16665: F, t16667: F, t16669: F, t16673: F) -> (F, F, F) {
    let t16678 = 4.0 / 27.0 * t519 * t3402 * t2497 * t1245 * t940;
    let t16680 = 8.0 / 45.0 * t2171 * t5261;
    let t16681 = -t16634 - t16637 - t16640 + t16643 + t16647 - t16649 - t16651 + t16653 - t16656 + t16660 - t16665 + t16667 + t16669 - t16673 - t16678 - t16680;
    (t16678, t16680, t16681)
}
