//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1026/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1026<F: Float>(t42224: F, t42227: F, t42230: F, t42233: F, t42236: F, t42239: F, t42242: F, t42245: F, t42250: F, t42257: F, t42263: F, t42265: F, t48088: F, t48093: F, t48096: F, t48099: F, t48107: F, t48109: F, t48111: F, t48115: F) -> F {
    let t50893 = -t42224 - t42227 + F::cast_from(0.21450293971110256002e1_f64) * t48088 - t48093 - t42230 + t42233 + t42236 + t42239 - t42242 + t42245 + t42250 + t42257 - t48096 - t48099 + t48107 + t48109 - t48111 - t48115 - t42263 + t42265;
    t50893
}
