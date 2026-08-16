//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 761/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk761(t4804: f64, t4856: f64, t4857: f64, t5639: f64, t5643: f64, t5647: f64, t5649: f64, t5653: f64, t5657: f64, t5661: f64, t5664: f64, t3107: f64, t3110: f64, t3112: f64, t3128: f64, t3142: f64, t3144: f64, t3161: f64, t4812: f64, t4814: f64, t4860: f64, t4863: f64) -> (f64, f64) {
    let t5777 = -2.0_f64 / 3.0_f64 * t5639 - 3.0_f64 / 2.0_f64 * t5643 + t5647 + t5649 / 3.0_f64 + t5653 / 2.0_f64 + t5657 / 12.0_f64 - t5661 / 24.0_f64 + t4856 + t4857 - t5664 / 4.0_f64 + t4804 / 3.0_f64;
    let t5783 = -t4860 - t4812 / 6.0_f64 - 14.0_f64 / 9.0_f64 * t4814 - t4863 + t3107 - t3110 + t3112 / 6.0_f64 - t3128 / 12.0_f64 - t3142 - 7.0_f64 / 9.0_f64 * t3144 + t3161;
    (t5777, t5783)
}
