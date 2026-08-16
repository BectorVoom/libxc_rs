//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 249/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk249<F: Float>(t1161: F, t325: F, t45: F, t330: F) -> (F, F, F, F) {
    let t1195 = F::cast_from(0.92708333333333333333e-2_f64) * t1161;
    let t1201 = t45 * t325;
    let t1202 = t330 * t330;
    let t1203 = F::cast_from(1.0_f64) / t1202;
    (t1195, t1201, t1202, t1203)
}
