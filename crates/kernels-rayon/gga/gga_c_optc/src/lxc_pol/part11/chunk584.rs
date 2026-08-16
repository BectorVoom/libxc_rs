//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 584/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk584(t2382: f64, t4786: f64, t2386: f64, t3640: f64, t4770: f64, t4774: f64, t4778: f64) -> (f64, f64) {
    let t4787 = t2382 * t4786;
    let t4793 = t2386 + 2.0_f64 / 9.0_f64 * t3640 - 2.0_f64 / 9.0_f64 * t4770 + 2.0_f64 / 3.0_f64 * t4774 - t4778 / 3.0_f64;
    (t4787, t4793)
}
