//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1163/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1163(t1897: f64, t1901: f64, t47322: f64, t13921: f64, t7137: f64, t7129: f64, t2508: f64, t2580: f64, t47220: f64, t43203: f64, t43204: f64, t43205: f64, t43206: f64, t43207: f64, t43208: f64, t43209: f64) -> f64 {
    let t47708 = 0.76905262301422242837e-2_f64 * t1897 * t1901 * t47322;
    let t47709 = t7137 * t13921;
    let t47711 = t7129 * t13921;
    let t47714 = t2508 * t2580 * t47220;
    let t47716 = t47708 + 0.20508069947045931423e-1_f64 * t47709 + 0.15381052460284448567e-1_f64 * t47711 + 0.15381052460284448567e-1_f64 * t47714 + t43203 - t43204 - t43205 + t43206 + t43207 + t43208 + t43209;
    t47716
}
