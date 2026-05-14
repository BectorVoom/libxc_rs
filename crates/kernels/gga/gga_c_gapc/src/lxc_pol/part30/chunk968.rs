//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 968/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk968<F: Float>(t128: F, t314: F, t786: F, t3327: F, t33655: F, t7451: F, t15507: F, t22: F, t5: F, t18679: F, t2763: F, t3699: F, t7730: F, t1899: F, t277: F, t26597: F, t2660: F) -> (F, F, F, F, F) {
    let t33657 = t314 * t128;
    let t33658 = t33657 * t786;
    let t33660 = t7451 * t33655 * t3327 * t33658;
    let t33666 = 1.0 / t22 / t15507 * M_PI * t5;
    let t33670 = t3699 * t18679 * t2763 * t7730;
    let t33671 = t277 * t1899 * t33666 * t33670;
    let t33673 = t2660 * t26597;
    (t33658, t33660, t33666, t33671, t33673)
}
