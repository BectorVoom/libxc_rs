//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 566/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk566(t4623: f64, t6: f64, t2024: f64, t161: f64, t1256: f64, t3360: f64, t2034: f64) -> (f64, f64, f64, f64, f64) {
    let t4624 = t6 * t4623;
    let t4625 = t4624 * t2024;
    let t4626 = t161 * t4625;
    let t4630 = t3360 * t1256;
    let t4631 = t2034 * t4630;
    (t4624, t4625, t4626, t4630, t4631)
}
