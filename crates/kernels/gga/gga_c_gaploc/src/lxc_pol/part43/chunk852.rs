//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 852/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk852<F: Float>(t2386: F, t42149: F, t204: F, t2476: F, t41738: F, t10615: F, t1423: F, t3129: F, t12871: F, t8155: F, t8158: F, t40372: F) -> (F, F, F, F, F, F) {
    let t42151 = F::new(0.53625734927775640005e1) * t42149 * t2386;
    let t42154 = F::new(0.92023022289409799224e1) * t2476 * t204 * t41738;
    let t42156 = t10615 * t1423 * t3129;
    let t42157 = F::new(0.17875244975925213335e0) * t42156;
    let t42159 = F::new(0.10725146985555128001e1) * t8155 * t12871;
    let t42161 = F::new(0.10725146985555128001e1) * t8158 * t12871;
    let t42168 = F::new(0.63904876589867916127e-1) * t40372;
    (t42151, t42154, t42157, t42159, t42161, t42168)
}
