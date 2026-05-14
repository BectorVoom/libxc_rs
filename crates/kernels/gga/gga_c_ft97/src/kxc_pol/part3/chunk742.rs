//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 742/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk742<F: Float>(t1017: F, t3590: F, t574: F, t4839: F, t558: F, t12680: F, t3430: F, t3435: F, t1045: F, t2097: F, t3441: F, t1060: F, t3408: F, t4714: F, t616: F, t167: F, t16919: F) -> (F, F, F, F, F, F, F, F) {
    let t17151 = t574 * t3590 * t1017;
    let t17155 = t574 * t4839 * t558;
    let t17158 = t12680 * t3430;
    let t17161 = t12680 * t3435;
    let t17164 = t2097 * t1045;
    let t17165 = t17164 * t3441;
    let t17170 = t574 * t1060 * t3408;
    let t17174 = t574 * t616 * t4714;
    let t17178 = t574 * t167 * t16919;
    (t17151, t17155, t17158, t17161, t17165, t17170, t17174, t17178)
}
