//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1971/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1971<F: Float>(t1329: F, t80775: F, t22822: F, t281: F, t6924: F, t22794: F, t120: F, t22816: F, t22814: F, t22855: F, t22823: F, t3862: F, t6940: F) -> (F, F, F, F, F, F, F, F, F) {
    let t80776 = t80775 * t1329;
    let t80779 = t22822 * t6924 * t281;
    let t80780 = t80779 * t22794;
    let t80782 = t22816 * t120;
    let t80783 = t22814 * t80782;
    let t80784 = t80783 * t22855;
    let t80791 = t22823 * t281;
    let t80792 = t80791 * t22855;
    let t80794 = t6940 * t3862;
    (t80776, t80779, t80780, t80782, t80783, t80784, t80791, t80792, t80794)
}
