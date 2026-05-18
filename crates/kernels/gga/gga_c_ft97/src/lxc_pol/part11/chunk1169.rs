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
    let t44769 = F::new(4.0) / F::new(27.0) * t43506;
    let t44771 = F::new(8.0) / F::new(9.0) * t43519;
    let t44775 = F::new(8.0) / F::new(81.0) * t43534;
    let t44776 = F::new(140.0) / F::new(243.0) * t43537;
    let t44778 = F::new(56.0) / F::new(81.0) * t43933;
    let t44779 = F::new(4.0) / F::new(9.0) * t43936;
    let t44781 = -F::new(6.0) * t43503 - t44769 - F::new(4.0) / F::new(3.0) * t43511 + t43516 + t44771 + F::new(4.0) / F::new(3.0) * t43522 + F::new(4.0) * t43528 + F::new(2.0) / F::new(9.0) * t43531 - t44775 + t44776 - t43930 / F::new(6.0) + t44778 - t44779 + F::new(4.0) / F::new(9.0) * t43940;
    t44781
}
