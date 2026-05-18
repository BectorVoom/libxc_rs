//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1054/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1054<F: Float>(t13254: F, t6402: F, t38143: F, t9035: F, t13271: F, t13282: F, t6484: F, t11868: F, t11984: F, t13287: F, t6416: F, t13173: F, t2119: F) -> (F, F, F, F, F, F, F) {
    let t45345 = t6402 * t13254;
    let t45351 = t9035 * t38143;
    let t45353 = t6402 * t13271;
    let t45381 = t6484 * t13282;
    let t45400 = t11984 * t11868;
    let t45408 = t6416 * t13287;
    let t45410 = t13173 * t2119;
    (t45345, t45351, t45353, t45381, t45400, t45408, t45410)
}
