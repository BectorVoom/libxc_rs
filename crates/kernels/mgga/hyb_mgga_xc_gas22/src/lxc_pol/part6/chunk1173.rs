//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1173/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1173<F: Float>(t3788: F, t9597: F, t1117: F, t9602: F, t2867: F, t9548: F, t1159: F, t22506: F, t524: F, t1143: F, t9574: F, t3756: F, t7774: F, t532: F, t22512: F, t536: F) -> (F, F, F, F, F, F, F, F) {
    let t26333 = t3788 * t9597;
    let t26345 = t1117 * t9602;
    let t26403 = t2867 * t9548;
    let t26409 = t524 * t22506 * t1159;
    let t26416 = t1143 * t9574;
    let t26421 = t7774 * t3756;
    let t26425 = t7774 * t532;
    let t26429 = t536 * t22512 * t1159;
    (t26333, t26345, t26403, t26409, t26416, t26421, t26425, t26429)
}
