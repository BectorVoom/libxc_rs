//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1169/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1169<F: Float>(t43506: F, t43519: F, t43534: F, t43537: F, t43933: F, t43936: F, t43503: F, t43511: F, t43516: F, t43522: F, t43528: F, t43531: F, t43930: F, t43940: F) -> F {
    let t44769 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t43506;
    let t44771 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43519;
    let t44775 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t43534;
    let t44776 = F::cast_from(140.0_f64) / F::cast_from(243.0_f64) * t43537;
    let t44778 = F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t43933;
    let t44779 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43936;
    let t44781 = -F::cast_from(6.0_f64) * t43503 - t44769 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43511 + t43516 + t44771 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43522 + F::cast_from(4.0_f64) * t43528 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t43531 - t44775 + t44776 - t43930 / F::cast_from(6.0_f64) + t44778 - t44779 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43940;
    t44781
}
