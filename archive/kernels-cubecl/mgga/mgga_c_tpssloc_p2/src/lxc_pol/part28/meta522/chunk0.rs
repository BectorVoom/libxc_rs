//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1770/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1770<F: Float>(t22814: F, t80782: F, t22855: F, t236: F, t3791: F, t22705: F, t22852: F, t550: F, t22823: F, t281: F, t3862: F, t6940: F) -> (F, F, F, F, F, F, F) {
    let t80783 = t22814 * t80782;
    let t80784 = t80783 * t22855;
    let t80786 = t236 * t3791;
    let t80789 = t22852 * t22705 * t80786 * t550;
    let t80791 = t22823 * t281;
    let t80792 = t80791 * t22855;
    let t80794 = t6940 * t3862;
    (t80783, t80784, t80786, t80789, t80791, t80792, t80794)
}
