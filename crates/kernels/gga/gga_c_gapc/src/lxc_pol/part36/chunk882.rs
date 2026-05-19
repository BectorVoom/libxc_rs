//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 882/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk882<F: Float>(t9048: F, t9051: F, t9054: F, t9057: F, t9062: F, t9064: F, t9069: F, t9073: F, t9076: F, t9081: F, t9085: F, t9088: F, t9093: F) -> F {
    let t10693 = F::cast_from(0.11255061864162936194e-7_f64) * t9048 + F::cast_from(0.11255061864162936194e-6_f64) * t9051 + F::cast_from(0.66704999981605668513e-8_f64) * t9054 - F::cast_from(0.34752370105806885418e-3_f64) * t9057 + F::cast_from(0.51564945349389680439e-8_f64) * t9062 - F::cast_from(0.9275345110817126956e-4_f64) * t9064 - F::cast_from(0.84540905957968605064e-6_f64) * t9069 + F::cast_from(0.33765185592488808582e-6_f64) * t9073 + F::cast_from(0.67530371184977617164e-6_f64) * t9076 + F::cast_from(0.33765185592488808582e-6_f64) * t9081 - F::cast_from(0.34752370105806885418e-3_f64) * t9085 + F::cast_from(0.51491428373437201896e-5_f64) * t9088 - F::cast_from(0.35580446990188463585e-8_f64) * t9093;
    t10693
}
