//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 648/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk648<F: Float>(t2475: F, t159: F, t167: F) -> (F, F, F, F) {
    let t2476 = F::cast_from(1.0_f64) / t2475;
    let t2477 = t159 * t2476;
    let t2478 = t167 * t167;
    let t2479 = F::cast_from(1.0_f64) / t2478;
    (t2476, t2477, t2478, t2479)
}
