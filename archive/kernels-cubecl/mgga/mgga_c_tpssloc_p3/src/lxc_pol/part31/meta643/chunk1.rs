//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1912/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1912<F: Float>(t23270: F, t258: F, t5527: F, t776: F, t87642: F, t6552: F, t7479: F, t87782: F, t2717: F, t5636: F, t22986: F, t5544: F) -> (F, F, F, F) {
    let t98153 = t87642 * t23270 * t258 * t5527 * t776;
    let t98158 = t6552 * t87782 * t7479;
    let t98161 = t2717 * t5636;
    let t98164 = t22986 * t23270 * t98161 * t776;
    let t98169 = t258 * t5544;
    (t98153, t98158, t98164, t98169)
}
