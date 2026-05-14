//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1086/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1086<F: Float>(t22632: F, t22810: F, t5611: F, t22825: F, t5517: F, t45566: F, t5597: F, t12: F, t14: F, t1675: F, t172: F, t22766: F) -> (F, F, F, F, F, F) {
    let t92388 = t22632 * t22810;
    let t92389 = t5611 * t92388;
    let t92399 = t5517 * t22825;
    let t92425 = t45566 * t5597;
    let t92429 = t12 * t1675 * t14;
    let t92433 = t22766 * t172;
    (t92388, t92389, t92399, t92425, t92429, t92433)
}
