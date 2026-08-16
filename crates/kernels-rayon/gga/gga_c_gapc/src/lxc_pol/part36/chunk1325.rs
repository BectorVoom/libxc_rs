//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1325/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1325(t2869: f64, t37339: f64, t37342: f64, t37344: f64, t37346: f64, t37349: f64, t37352: f64, t37354: f64, t37356: f64, t37478: f64, t37644: f64, t37649: f64, t38518: f64, t38858: f64, t3899: f64) -> f64 {
    let t38886 = t2869 * t3899 - t37339 - t37342 - t37344 + t37346 + t37349 - t37352 + t37354 + t37356 + t37478 - t37644 + t37649 + t38518 + 2.0_f64 * t38858;
    t38886
}
