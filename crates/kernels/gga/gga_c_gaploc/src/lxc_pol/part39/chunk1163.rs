//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1163/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1163<F: Float>(t1897: F, t1901: F, t47322: F, t13921: F, t7137: F, t7129: F, t2508: F, t2580: F, t47220: F, t43203: F, t43204: F, t43205: F, t43206: F, t43207: F, t43208: F, t43209: F) -> F {
    let t47708 = F::new(0.76905262301422242837e-2) * t1897 * t1901 * t47322;
    let t47709 = t7137 * t13921;
    let t47711 = t7129 * t13921;
    let t47714 = t2508 * t2580 * t47220;
    let t47716 = t47708 + F::new(0.20508069947045931423e-1) * t47709 + F::new(0.15381052460284448567e-1) * t47711 + F::new(0.15381052460284448567e-1) * t47714 + t43203 - t43204 - t43205 + t43206 + t43207 + t43208 + t43209;
    t47716
}
