//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 912/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk912<F: Float>(t225: F, t23226: F, t23228: F, t214: F, t2710: F, t794: F, t852: F, t213: F, t23202: F, t23211: F, t1081: F, t2752: F) -> (F, F, F, F, F, F, F, F) {
    let t82071 = t23226 * t225;
    let t82074 = t23228 * t225;
    let t82124 = t214 * t2710;
    let t82133 = t794 * t852;
    let t82159 = t213 * t852 * t225;
    let t82197 = t23202 * t225;
    let t82287 = t23211 * t225;
    let t83555 = t2752 * t1081;
    (t82071, t82074, t82124, t82133, t82159, t82197, t82287, t83555)
}
