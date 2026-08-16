//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1888/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1888<F: Float>(t1339: F, t550: F, t57091: F, t6936: F, t19890: F, t26309: F, t236: F, t6387: F, t22705: F, t22852: F, t19805: F, t2002: F, t559: F) -> (F, F, F, F, F) {
    let t97307 = t6936 * t1339 * t57091 * t550;
    let t97310 = t26309 * t19890;
    let t97312 = t236 * t6387;
    let t97315 = t22852 * t22705 * t97312 * t550;
    let t97318 = t19805 * t2002 * t559;
    (t97307, t97310, t97312, t97315, t97318)
}
