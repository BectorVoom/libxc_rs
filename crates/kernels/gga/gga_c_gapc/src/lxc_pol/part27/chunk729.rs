//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 729/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk729<F: Float>(t9259: F, t9262: F, t9214: F, t9217: F, t9220: F, t9224: F, t9226: F, t9230: F, t9233: F, t9235: F, t9239: F, t9242: F, t9250: F, t9257: F, t144: F, t8785: F) -> (F, F) {
    let t9263 = t9259 * t9262;
    let t9265 = 0.86880925264517213544e-4 * t9214 - 0.14480154210752868924e-5 * t9217 - 0.25745714186718600948e-5 * t9220 - 0.25745714186718600948e-5 * t9224 + 0.10821235962619981449e-3 * t9226 + 0.20241536458333333334e-4 * t9230 + 0.10120768229166666667e-3 * t9233 - 0.30660168560756614104e-3 * t9235 - 0.11101451561577199508e-4 * t9239 - 0.10120768229166666667e-4 * t9242 - 0.14591718745976239987e-8 * t9250 + 0.49240895655712845848e-7 * t9257 + 0.98481791311425691697e-7 * t9263;
    let t9267 = t8785 * t144;
    (t9265, t9267)
}
