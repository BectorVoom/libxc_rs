//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 569/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk569(t127: f64, t4649: f64, t6: f64, t161: f64, t4624: f64, t141: f64, t4599: f64, t2087: f64) -> (f64, f64, f64, f64, f64) {
    let t4651 = t6 * t4649 * t127;
    let t4652 = t161 * t4651;
    let t4655 = t4624 * t127;
    let t4656 = t161 * t4655;
    let t4660 = t141 * t4599;
    let t4661 = t2087 * t4660;
    (t4651, t4652, t4655, t4656, t4661)
}
