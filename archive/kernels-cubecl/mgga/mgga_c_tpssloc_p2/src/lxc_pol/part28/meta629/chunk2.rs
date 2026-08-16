//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1972/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1972<F: Float>(t86911: F, t86916: F, t86928: F, t86940: F, t86942: F, t13029: F, t2047: F, t259: F, t26700: F, t2743: F, t4142: F, t7084: F, t7842: F, t82079: F, t82082: F, t82087: F, t86933: F, t9590: F) -> (F, F, F) {
    let t92402 = F::cast_from(0.52089578783527170489e-1_f64) * t86911;
    let t92406 = F::cast_from(0.3289868133696452873e-1_f64) * t86916;
    let t92415 = F::cast_from(0.16449340668482264365e-1_f64) * t86928;
    let t92425 = F::cast_from(0.16449340668482264365e-1_f64) * t86940;
    let t92426 = F::cast_from(0.76763589786250567036e-1_f64) * t86942;
    let t92428 = F::cast_from(0.82246703342411321825e-2_f64) * t82079 - t92415 + F::cast_from(0.3289868133696452873e-1_f64) * t82082 - F::cast_from(0.3289868133696452873e-1_f64) * t82087 - t9590 * t7842 + F::cast_from(2.0_f64) * t4142 * t7084 * t259 + F::cast_from(0.6579736267392905746e-1_f64) * t86933 + t13029 * t2047 * t259 + t92425 + t92426 - t26700 * t2743;
    (t92402, t92406, t92428)
}
