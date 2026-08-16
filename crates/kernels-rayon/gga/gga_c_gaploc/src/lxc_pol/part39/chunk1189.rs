//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1189/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1189(t13801: f64, t1641: f64, t41960: f64, t41962: f64, t41968: f64, t41970: f64, t41972: f64, t41973: f64, t47976: f64, t47978: f64, t47980: f64, t47984: f64, t47987: f64) -> f64 {
    let t47989 = t1641 * t13801;
    let t47992 = 0.14896037479937677779e-1_f64 * t41960 + 0.14896037479937677779e-1_f64 * t41962 + 0.14896037479937677779e-1_f64 * t47976 + 0.14896037479937677779e-1_f64 * t47978 - 0.14896037479937677779e-1_f64 * t47980 - 0.14896037479937677779e-1_f64 * t47984 - 0.71500979903700853338e0_f64 * t47987 - 0.46011511144704899612e1_f64 * t47989 + t41968 + 0.46011511144704899612e1_f64 * t41970 - t41972 - t41973;
    t47992
}
