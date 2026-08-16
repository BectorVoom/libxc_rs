//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1034/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1034(t2153: f64, t2206: f64, t5692: f64, t8: f64, t5: f64, t17890: f64, t277: f64, t575: f64, t8596: f64, t2468: f64, t3263: f64, t2902: f64, t423: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24625 = t2153 * t2206;
    let t24759 = 1.0_f64 / t8 / t5692;
    let t24760 = t5 * t24759;
    let t24761 = t277 * t17890;
    let t24906 = t8596 * t575;
    let t24915 = t3263 * t2468;
    let t24980 = t2902 * t423;
    (t24625, t24759, t24760, t24761, t24906, t24915, t24980)
}
