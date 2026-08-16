//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1625/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1625<F: Float>(t4017: F, t71: F, t12568: F, t33: F, t3953: F, t608: F, t1437: F, t641: F, t72: F, t4021: F, t79: F, t1410: F, t2235: F) -> (F, F, F, F, F, F) {
    let t26024 = t71 * t4017;
    let t26028 = t12568 * t33;
    let t26055 = t3953 * t608;
    let t26062 = t641 * t1437;
    let t26063 = t72 * t26062;
    let t26066 = t79 * t4021;
    let t26067 = t72 * t26066;
    let t26070 = t2235 * t1410;
    (t26024, t26028, t26055, t26063, t26067, t26070)
}
