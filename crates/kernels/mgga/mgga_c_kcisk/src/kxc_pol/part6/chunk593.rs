//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 593/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk593<F: Float>(t529: F, t1459: F, t8286: F, t2331: F, t4350: F, t41: F, t7828: F, t2153: F, t2308: F, t382: F, t525: F, t526: F, t8011: F, t8015: F) -> (F, F, F, F, F) {
    let t530 = t529 < -F::cast_from(0.66725e-1_f64);
    let t8287 = t1459 * t8286;
    let t8288 = t2331 * t2331;
    let t8289 = t8288 * t4350;
    let t8292 = t7828 * t41;
    let t8306 = piecewise3::<F>(t530, F::cast_from(0.0_f64), F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t525 * t8292 * t382 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t525 * t2308 * t2153 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t525 * t526 * t8011 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t525 * t526 * t8015);
    (t8287, t8288, t8289, t8292, t8306)
}
