//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1946/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1946<F: Float>(t16949: F, t221: F, t25154: F, t25119: F, t841: F, t25038: F, t25248: F, t776: F, t98422: F, t23110: F, t23185: F, t28321: F) -> (F, F, F, F) {
    let t98868 = t25154 * t221 * t16949;
    let t98871 = t25119 * t841 * t16949;
    let t98881 = t25038 * t25248 * t98422 * t776;
    let t98884 = t23185 * t23110 * t28321;
    (t98868, t98871, t98881, t98884)
}
