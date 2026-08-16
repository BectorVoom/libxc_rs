//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1897/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1897<F: Float>(t22779: F, t28060: F, t19661: F, t1992: F, t22897: F, t19736: F, t22892: F, t22893: F, t28138: F, t28116: F, t81228: F, t81326: F) -> (F, F, F, F, F) {
    let t97463 = t22779 * t28060;
    let t97488 = t1992 * t22897 * t19661;
    let t97491 = t1992 * t22897 * t19736;
    let t97494 = t22892 * t22893 * t28138;
    let t97503 = t81228 * t81326 * t28116;
    (t97463, t97488, t97491, t97494, t97503)
}
