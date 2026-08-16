//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1886/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1886<F: Float>(t1369: F, t97265: F, t1339: F, t1824: F, t22827: F, t5187: F, t550: F, t74677: F, t1307: F, t3788: F, t6388: F, t22783: F, t6427: F) -> (F, F, F, F, F) {
    let t97266 = t97265 * t1369;
    let t97273 = t22827 * t1339 * t5187 * t1824 * t550;
    let t97277 = t22827 * t1339 * t74677 * t550;
    let t97281 = t22827 * t3788 * t6388 * t1307;
    let t97283 = t22783 * t6427;
    (t97266, t97273, t97277, t97281, t97283)
}
