//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 993/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk993<F: Float>(t15: F, t588: F, t11: F, t2: F, t22: F, t2224: F, t27: F, t584: F, t20: F, t596: F, t12: F, t583: F) -> (F, F, F, F, F, F) {
    let t10275 = F::new(24.0) * t15 * t588;
    let t10276 = t11 * t2;
    let t10278 = F::new(24.0) * t10276 * t22;
    let t10279 = t2224 * t588;
    let t10281 = t584 * t27;
    let t10284 = F::new(120.0) * t20 * t596;
    let t10285 = t12 * t583;
    (t10275, t10278, t10279, t10281, t10284, t10285)
}
