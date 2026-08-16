//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2156/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2156<F: Float>(t25192: F, t81651: F, t82074: F, t225: F, t25220: F, t82259: F, t6552: F, t6555: F, t87782: F, t23270: F, t25038: F, t25191: F, t87036: F) -> (F, F, F, F, F) {
    let t87835 = t81651 * t82074 * t25192;
    let t87836 = F::cast_from(0.16449340668482264365e-1_f64) * t87835;
    let t87837 = t25220 * t225;
    let t87847 = F::cast_from(0.12793931631041761173e0_f64) * t82259;
    let t87861 = t6552 * t87782 * t6555;
    let t87866 = t25038 * t23270 * t25191 * t87036;
    (t87836, t87837, t87847, t87861, t87866)
}
