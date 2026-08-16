//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 867/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk867<F: Float>(t42156: F, t12871: F, t8155: F, t8158: F, t41878: F, t6717: F, t6914: F, t10532: F, t10533: F, t40372: F, t40377: F, t40392: F) -> (F, F, F, F, F, F, F, F) {
    let t42157 = F::cast_from(0.17875244975925213335e0_f64) * t42156;
    let t42159 = F::cast_from(0.10725146985555128001e1_f64) * t8155 * t12871;
    let t42161 = F::cast_from(0.10725146985555128001e1_f64) * t8158 * t12871;
    let t42163 = t6914 * t6717 * t41878;
    let t42166 = t10532 * t10533 * t41878;
    let t42168 = F::cast_from(0.63904876589867916127e-1_f64) * t40372;
    let t42170 = F::cast_from(0.19171462976960374838e0_f64) * t40377;
    let t42172 = F::cast_from(0.15337170381568299871e1_f64) * t40392;
    (t42157, t42159, t42161, t42163, t42166, t42168, t42170, t42172)
}
