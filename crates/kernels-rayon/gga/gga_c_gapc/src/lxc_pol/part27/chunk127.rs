//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 127/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk127(t76: f64, t96: f64, t131: f64, t5: f64, t83: f64, t7: f64, t87: f64) -> (f64, f64, f64, f64) {
    let t408 = t76 * t96;
    let t410 = t5 * t131;
    let t411 = t83 * t410;
    let t413 = t87 * t7;
    (t408, t410, t411, t413)
}
