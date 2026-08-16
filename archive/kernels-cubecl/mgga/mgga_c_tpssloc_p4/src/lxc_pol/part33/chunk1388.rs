//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1388/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1388<F: Float>(t20479: F, t6952: F, t1831: F, t97265: F, t1998: F, t20356: F, t236: F, t80894: F, t1799: F, t22827: F, t3788: F, t6388: F) -> (F, F, F, F) {
    let t107133 = t6952 * t20479;
    let t107135 = t97265 * t1831;
    let t107139 = t80894 * t1998 * t236 * t20356;
    let t107143 = t22827 * t3788 * t6388 * t1799;
    (t107133, t107135, t107139, t107143)
}
