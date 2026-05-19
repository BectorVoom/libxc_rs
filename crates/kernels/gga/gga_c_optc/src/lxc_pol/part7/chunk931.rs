//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 931/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk931<F: Float>(t1067: F, t1095: F, t2937: F, t2974: F, t402: F, t8560: F, t8564: F, t8571: F, t8574: F, t8576: F, t8579: F, t8585: F, t8682: F, t8691: F, t8806: F, t8809: F, t8843: F, t8848: F, t8851: F, t8854: F, t8869: F, t8882: F, t8898: F) -> F {
    let t8899 = -t8571 - t8576 - t8579 + t8585 - t8682 - t8691 - F::new(6.0) * t8806 * t2937 + F::new(6.0) * t2974 * t8809 + F::new(1.0) * t1067 * t8843 + F::cast_from(0.20691336878655965246e4_f64) * t8848 * t8851 + F::cast_from(0.17544670192365612213e1_f64) * t8854 * t1095 + t8560 - t8564 - t8574 - F::cast_from(0.19751789702565206229e-1_f64) * t8869 - F::new(0.3109e-1) * t8882 * t402 + t8898;
    t8899
}
