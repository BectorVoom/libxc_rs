//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2166/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2166<F: Float>(t1339: F, t1824: F, t22827: F, t5187: F, t550: F, t74677: F, t1307: F, t3788: F, t6388: F, t22783: F, t6427: F, t26288: F, t57172: F) -> (F, F, F, F, F) {
    let t97273 = t22827 * t1339 * t5187 * t1824 * t550;
    let t97277 = t22827 * t1339 * t74677 * t550;
    let t97281 = t22827 * t3788 * t6388 * t1307;
    let t97283 = t22783 * t6427;
    let t97287 = t26288 * t1339 * t57172 * t550;
    (t97273, t97277, t97281, t97283, t97287)
}
