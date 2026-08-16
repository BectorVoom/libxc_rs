//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2093/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2093<F: Float>(t25038: F, t25040: F, t82159: F, t23030: F, t25035: F, t23228: F, t7479: F, t81573: F, t22986: F, t23270: F, t25191: F, t2742: F) -> (F, F, F, F) {
    let t86909 = t25038 * t82159 * t25040;
    let t86911 = t23030 * t25035;
    let t86916 = t81573 * t23228 * t7479;
    let t86923 = t22986 * t23270 * t25191 * t2742;
    (t86909, t86911, t86916, t86923)
}
