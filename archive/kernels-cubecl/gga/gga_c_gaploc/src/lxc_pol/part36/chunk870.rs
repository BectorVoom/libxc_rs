//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 870/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk870<F: Float>(t12953: F, t31054: F, t12986: F, t2464: F, t2487: F, t35204: F, t9346: F, t204: F, t41965: F, t587: F, t10156: F, t3377: F, t524: F) -> (F, F, F, F, F) {
    let t42199 = t31054 * t12953;
    let t42200 = F::cast_from(0.11502877786176224903e1_f64) * t42199;
    let t42202 = t2487 * t2464 * t12986;
    let t42203 = F::cast_from(0.63904876589867916128e-1_f64) * t42202;
    let t42205 = F::cast_from(0.21450293971110256001e2_f64) * t35204 * t9346;
    let t42208 = F::cast_from(0.92023022289409799224e1_f64) * t587 * t204 * t41965;
    let t42210 = t524 * t10156 * t3377;
    (t42200, t42203, t42205, t42208, t42210)
}
