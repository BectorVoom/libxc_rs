//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 792/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk792(t144: f64, t8785: f64, t1734: f64, t1030: f64, t1672: f64, t3142: f64, t4: f64, t5: f64) -> (f64, f64, f64) {
    let t9267 = t8785 * t144;
    let t9268 = t1734 * t9267;
    let t9269 = t1030 * t9268;
    let t9272 = t1672 * t5 * t3142 * t4;
    (t9267, t9269, t9272)
}
