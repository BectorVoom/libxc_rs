//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2428/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2428(t2403: f64, t2830: f64, t909: f64, t9709: f64, t2833: f64, t2827: f64, t10213: f64, t241: f64, t41654: f64, t270: f64, t276: f64, t39267: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41831 = t2403 * t2830;
    let t41863 = t9709 * t909;
    let t41870 = t2403 * t2833;
    let t41872 = t2403 * t2827;
    let t41880 = t241 * t10213;
    let t41904 = 280.0_f64 / 81.0_f64 * t41654;
    let t41935 = 1.0_f64 / t276 / t39267 / t270 / 96.0_f64;
    (t41831, t41863, t41870, t41872, t41880, t41904, t41935)
}
