//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 625/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk625<F: Float>(t1649: F, t3679: F, t1643: F, t1629: F, t188: F, t116: F, t205: F, t1033: F, t435: F) -> (F, F, F, F, F, F) {
    let t3680 = t3679 * t1649;
    let t3681 = t1643 * t3680;
    let t3683 = t1629 * t188;
    let t3684 = t116 * t3683;
    let t3685 = t3684 * t205;
    let t3687 = t435 * t1033;
    (t3680, t3681, t3683, t3684, t3685, t3687)
}
