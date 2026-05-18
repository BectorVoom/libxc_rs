//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 984/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk984<F: Float>(t14420: F, t14436: F, t270: F, t42960: F, t42967: F, t42970: F, t44751: F, t44756: F, t44759: F, t44762: F, t44764: F, t44776: F, t44780: F, t47594: F, t47597: F, t47607: F, t47610: F, t50183: F, t650: F, t681: F, t738: F) -> F {
    let t50356 = -F::new(0.89722806018325949978e-2) * t42960 + t44751 + F::new(0.1281754371690370714e-2) * t47594 - F::new(0.76905262301422242837e-2) * t42967 - F::new(0.25635087433807414279e-2) * t42970 + t44756 - t44759 - F::new(0.1281754371690370714e-2) * t47597 - F::new(0.3845263115071112142e-2) * t47607 - t44762 + F::new(0.1281754371690370714e-2) * t44764 + F::new(0.2563508743380741428e-2) * t47610 - t44776 - t44780 + F::new(0.10254034973522965712e-1) * t650 * t14436 - F::new(0.76905262301422242837e-2) * t681 * t14420 - F::new(0.76905262301422242837e-2) * t270 * t738 * t50183 - F::new(0.10254034973522965712e-1) * t650 * t14420;
    t50356
}
