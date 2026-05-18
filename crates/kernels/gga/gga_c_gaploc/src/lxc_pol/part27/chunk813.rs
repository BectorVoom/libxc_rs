//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 813/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk813<F: Float>(t2365: F, t7778: F, t5680: F, t5688: F, t959: F, t325: F, t883: F, t900: F, t6117: F, t1710: F, t2610: F, t2033: F) -> (F, F, F, F) {
    let t7779 = t2365 * t7778;
    let t7780 = t5680 * t7779;
    let t7782 = t5688 * t959;
    let t7784 = t883 * t325;
    let t7785 = t900 * t7784;
    let t7786 = t6117 * t7785;
    let t7788 = t2610 * t1710;
    let t7789 = t2365 * t7788;
    let t7790 = t2033 * t7789;
    (t7780, t7782, t7786, t7790)
}
