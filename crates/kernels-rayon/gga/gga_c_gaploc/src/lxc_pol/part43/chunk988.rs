//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 988/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk988(t1841: f64, t2576: f64, t39347: f64, t13937: f64, t731: f64, t1897: f64, t1901: f64, t47322: f64, t13921: f64, t7137: f64, t7129: f64, t2508: f64, t2580: f64, t47220: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47696 = t1841 * t39347 * t2576;
    let t47702 = t731 * t13937;
    let t47708 = 0.76905262301422242837e-2_f64 * t1897 * t1901 * t47322;
    let t47709 = t7137 * t13921;
    let t47711 = t7129 * t13921;
    let t47714 = t2508 * t2580 * t47220;
    (t47696, t47702, t47708, t47709, t47711, t47714)
}
