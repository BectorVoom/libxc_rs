//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2167/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2167<F: Float>(t25319: F, t2553: F, t6552: F, t6637: F, t252: F, t87230: F, t13230: F, t87052: F, t23168: F, t25321: F, t25284: F, t6579: F) -> (F, F, F, F) {
    let t87527 = t6552 * t6637 * t25319 * t2553;
    let t87529 = t87230 * t252;
    let t87531 = t87052 * t87529 * t13230;
    let t87533 = t23168 * t25321;
    let t87534 = F::cast_from(0.76763589786250567036e-1_f64) * t87533;
    let t87535 = t6579 * t25284;
    (t87527, t87531, t87534, t87535)
}
