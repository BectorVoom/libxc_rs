//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 984/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk984(t14420: f64, t14436: f64, t270: f64, t42960: f64, t42967: f64, t42970: f64, t44751: f64, t44756: f64, t44759: f64, t44762: f64, t44764: f64, t44776: f64, t44780: f64, t47594: f64, t47597: f64, t47607: f64, t47610: f64, t50183: f64, t650: f64, t681: f64, t738: f64) -> f64 {
    let t50356 = -0.89722806018325949978e-2_f64 * t42960 + t44751 + 0.1281754371690370714e-2_f64 * t47594 - 0.76905262301422242837e-2_f64 * t42967 - 0.25635087433807414279e-2_f64 * t42970 + t44756 - t44759 - 0.1281754371690370714e-2_f64 * t47597 - 0.3845263115071112142e-2_f64 * t47607 - t44762 + 0.1281754371690370714e-2_f64 * t44764 + 0.2563508743380741428e-2_f64 * t47610 - t44776 - t44780 + 0.10254034973522965712e-1_f64 * t650 * t14436 - 0.76905262301422242837e-2_f64 * t681 * t14420 - 0.76905262301422242837e-2_f64 * t270 * t738 * t50183 - 0.10254034973522965712e-1_f64 * t650 * t14420;
    t50356
}
