//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1023/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1023(t137: f64, t136: f64, t627: f64, t6896: f64, t130: f64, t131: f64, t142: f64, t20816: f64, t2003: f64, t2010: f64, t623: f64, t6944: f64) -> (f64, f64, f64, f64, f64) {
    let t22834 = t137 * t137;
    let t22835 = 1.0_f64 / t22834;
    let t22836 = t136 * t22835;
    let t22850 = t6896 * t627;
    let t22856 = 0.36717874996221960261e1_f64 * t130 * t131 * t20816 * t142;
    let t22889 = t2003 * t2010;
    let t22892 = t623 * t6944;
    (t22836, t22850, t22856, t22889, t22892)
}
