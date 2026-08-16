//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 931/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk931(t1067: f64, t1095: f64, t2937: f64, t2974: f64, t402: f64, t8560: f64, t8564: f64, t8571: f64, t8574: f64, t8576: f64, t8579: f64, t8585: f64, t8682: f64, t8691: f64, t8806: f64, t8809: f64, t8843: f64, t8848: f64, t8851: f64, t8854: f64, t8869: f64, t8882: f64, t8898: f64) -> f64 {
    let t8899 = -t8571 - t8576 - t8579 + t8585 - t8682 - t8691 - 6.0_f64 * t8806 * t2937 + 6.0_f64 * t2974 * t8809 + 1.0_f64 * t1067 * t8843 + 0.20691336878655965246e4_f64 * t8848 * t8851 + 0.17544670192365612213e1_f64 * t8854 * t1095 + t8560 - t8564 - t8574 - 0.19751789702565206229e-1_f64 * t8869 - 0.3109e-1_f64 * t8882 * t402 + t8898;
    t8899
}
