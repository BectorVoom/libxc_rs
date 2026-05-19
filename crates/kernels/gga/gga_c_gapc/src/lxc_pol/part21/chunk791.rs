//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 791/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk791<F: Float>(t9259: F, t9262: F, t9214: F, t9217: F, t9220: F, t9224: F, t9226: F, t9230: F, t9233: F, t9235: F, t9239: F, t9242: F, t9250: F, t9257: F) -> F {
    let t9263 = t9259 * t9262;
    let t9265 = F::cast_from(0.86880925264517213544e-4_f64) * t9214 - F::cast_from(0.14480154210752868924e-5_f64) * t9217 - F::cast_from(0.25745714186718600948e-5_f64) * t9220 - F::cast_from(0.25745714186718600948e-5_f64) * t9224 + F::cast_from(0.10821235962619981449e-3_f64) * t9226 + F::cast_from(0.20241536458333333334e-4_f64) * t9230 + F::cast_from(0.10120768229166666667e-3_f64) * t9233 - F::cast_from(0.30660168560756614104e-3_f64) * t9235 - F::cast_from(0.11101451561577199508e-4_f64) * t9239 - F::cast_from(0.10120768229166666667e-4_f64) * t9242 - F::cast_from(0.14591718745976239987e-8_f64) * t9250 + F::cast_from(0.49240895655712845848e-7_f64) * t9257 + F::cast_from(0.98481791311425691697e-7_f64) * t9263;
    t9265
}
