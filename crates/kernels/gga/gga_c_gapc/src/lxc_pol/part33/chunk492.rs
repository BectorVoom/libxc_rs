//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 492/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk492<F: Float>(t2859: F, t2867: F, t43: F, t385: F, t991: F, t426: F, t118: F, t632: F, t61: F, t126: F, t144: F) -> (F, F, F, F, F, F) {
    let t2869 = (t2859 + t2867) * t43;
    let t2874 = t385 * t991;
    let t2876 = t426 * t991;
    let t2878 = t632 * t118;
    let t2879 = t61 * t2878;
    let t2880 = t126 * t144;
    (t2869, t2874, t2876, t2878, t2879, t2880)
}
