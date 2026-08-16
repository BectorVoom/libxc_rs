//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1017/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1017(t1: f64, t12323: f64, t3: f64, t672: f64, t12829: f64, t230: f64, t12646: f64, t5493: f64, t639: f64, t12476: f64, t5125: f64, t587: f64) -> (f64, f64, f64, f64) {
    let t41334 = t12323 * t1 * t3 * t672;
    let t41339 = t12829 * t230;
    let t41359 = t639 * t5493 * t12646;
    let t41385 = t587 * t5125 * t12476;
    (t41334, t41339, t41359, t41385)
}
