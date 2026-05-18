//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 745/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk745<F: Float>(t15664: F, t1594: F, t3056: F, t930: F, t428: F, t4467: F, t374: F, t15657: F, t1631: F, t15630: F, t534: F, t383: F, t77: F) -> (F, F, F, F, F, F, F) {
    let t15665 = t1594 * t15664;
    let t15668 = t930 * t3056;
    let t15669 = t1594 * t15668;
    let t15673 = t4467 * t428;
    let t15674 = t374 * t15673;
    let t15677 = t1631 * t15657;
    let t15680 = t534 * t15630;
    let t15681 = t77 * t383;
    (t15665, t15668, t15669, t15674, t15677, t15680, t15681)
}
