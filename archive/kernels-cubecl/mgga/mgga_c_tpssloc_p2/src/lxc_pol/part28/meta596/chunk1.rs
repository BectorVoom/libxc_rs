//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1894/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1894<F: Float>(t12571: F, t608: F, t33: F, t46099: F, t2244: F, t3953: F, t1410: F, t9239: F, t2241: F, t72: F, t7431: F, t12648: F, t605: F) -> (F, F, F, F, F, F) {
    let t90114 = t12571 * t608;
    let t90121 = t46099 * t33;
    let t90132 = t3953 * t2244;
    let t90137 = t9239 * t1410;
    let t90141 = t72 * t7431 * t2241;
    let t90150 = t605 * t12648;
    (t90114, t90121, t90132, t90137, t90141, t90150)
}
