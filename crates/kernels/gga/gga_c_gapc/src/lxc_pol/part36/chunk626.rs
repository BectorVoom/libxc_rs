//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 626/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk626<F: Float>(t457: F, t682: F, t5741: F, t1927: F, t583: F, t623: F, t1393: F, t515: F, t1709: F, t618: F, t567: F, t635: F, t144: F, t1908: F) -> (F, F, F, F, F, F, F) {
    let t5742 = t682 * t457;
    let t5743 = t5741 * t5742;
    let t5799 = t1927 * t583;
    let t5803 = t1927 * t623;
    let t5856 = t1393 * t515;
    let t5858 = t618 * t1709;
    let t5918 = t635 * t567;
    let t5963 = t144 * t1908;
    (t5743, t5799, t5803, t5856, t5858, t5918, t5963)
}
