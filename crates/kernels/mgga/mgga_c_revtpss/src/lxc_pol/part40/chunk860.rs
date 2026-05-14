//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 860/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk860<F: Float>(t5205: F, t5206: F, t1196: F, t3358: F, t3546: F, t5044: F, t5049: F, t5054: F, t5058: F, t459: F) -> (F, F, F, F) {
    let t5207 = t5205 * t5206;
    let t5209 = 0.17315859105681463759e2 * t1196 * t5207;
    let t5215 = t3546 - 0.27777777777777777778e-2 * t3358 - 0.27777777777777777778e-2 * t5044 - 0.55555555555555555555e-2 * t5049 + 0.16666666666666666667e-1 * t5054 + 0.83333333333333333333e-2 * t5058;
    let t5216 = t5215 * t459;
    (t5207, t5209, t5215, t5216)
}
