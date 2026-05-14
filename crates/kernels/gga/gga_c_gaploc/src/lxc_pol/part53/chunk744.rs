//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 744/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk744<F: Float>(t12953: F, t31054: F, t12986: F, t2464: F, t2487: F, t35204: F, t9346: F, t204: F, t41965: F, t587: F, t31139: F, t544: F, t986: F, t2386: F, t10525: F, t10526: F) -> (F, F, F, F, F, F) {
    let t42199 = t31054 * t12953;
    let t42200 = 0.11502877786176224903e1 * t42199;
    let t42202 = t2487 * t2464 * t12986;
    let t42203 = 0.63904876589867916128e-1 * t42202;
    let t42205 = 0.21450293971110256001e2 * t35204 * t9346;
    let t42208 = 0.92023022289409799224e1 * t587 * t204 * t41965;
    let t42219 = t544 * t31139 * t986;
    let t42221 = 0.25025342966295298669e1 * t42219 * t2386;
    let t42224 = 0.21450293971110256001e1 * t10525 * t10526 * t41965;
    (t42200, t42203, t42205, t42208, t42221, t42224)
}
