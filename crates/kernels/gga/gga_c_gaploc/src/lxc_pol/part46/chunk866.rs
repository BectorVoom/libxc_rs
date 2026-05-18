//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 866/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk866<F: Float>(t40353: F, t12964: F, t2487: F, t6985: F, t9078: F, t986: F, t544: F, t2386: F, t204: F, t2476: F, t41738: F, t10615: F, t1423: F, t3129: F) -> (F, F, F, F, F, F) {
    let t42144 = F::new(0.11502877786176224903e1) * t40353;
    let t42146 = t2487 * t6985 * t12964;
    let t42148 = t9078 * t986;
    let t42149 = t544 * t42148;
    let t42151 = F::new(0.53625734927775640005e1) * t42149 * t2386;
    let t42154 = F::new(0.92023022289409799224e1) * t2476 * t204 * t41738;
    let t42156 = t10615 * t1423 * t3129;
    (t42144, t42146, t42148, t42151, t42154, t42156)
}
