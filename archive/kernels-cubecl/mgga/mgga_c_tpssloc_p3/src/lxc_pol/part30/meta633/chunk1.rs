//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2041/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2041<F: Float>(t87679: F, t25273: F, t6579: F, t244: F, t268: F, t6559: F, t25250: F, t87202: F, t25316: F, t82038: F, t23110: F, t23185: F, t25272: F) -> (F, F, F, F, F, F) {
    let t87680 = F::cast_from(0.16449340668482264365e-1_f64) * t87679;
    let t87709 = t6579 * t25273;
    let t87710 = F::cast_from(0.38381794893125283518e-1_f64) * t87709;
    let t87712 = t6559 * t244 * t268;
    let t87714 = t87712 * t87202 * t25250;
    let t87718 = t82038 * t25316;
    let t87729 = t23185 * t23110 * t25272;
    (t87680, t87710, t87712, t87714, t87718, t87729)
}
