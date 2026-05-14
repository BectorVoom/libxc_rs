//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 675/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk675<F: Float>(t496: F, t5783: F, t10: F, t127: F, t5744: F, t5749: F, t5751: F, t5753: F, t5755: F, t5759: F, t5764: F, t5768: F, t5771: F, t5776: F, t5779: F, t5780: F) -> (F,) {
    let t5784 = t496 * t5783;
    let t5786 = 9.0 / 2.0 * t496 * t10 * t5744 - t5749 - t5751 + t5753 - t5755 - t5759 - 0.146904e1 * t5764 + 0.220356e1 * t5768 + t5771 - t5776 - t5779 - 0.146904e1 * t127 * t5780 - 2.0 / 3.0 * t5784;
    (t5786,)
}
