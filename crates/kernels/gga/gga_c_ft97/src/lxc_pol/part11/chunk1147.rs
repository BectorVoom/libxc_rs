//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1147/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1147<F: Float>(t43537: F, t43511: F, t43516: F, t43519: F, t43522: F, t43528: F, t43531: F, t43534: F, t43541: F, t43551: F, t43926: F, t43930: F, t43933: F, t43936: F, t43940: F) -> F {
    let t44121 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t43537;
    let t44128 = -F::cast_from(8.0_f64) * t43511 + F::cast_from(6.0_f64) * t43516 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t43519 + F::cast_from(8.0_f64) * t43522 + F::cast_from(24.0_f64) * t43528 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43531 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t43534 + t44121 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t43541 - F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t43551 + t43926 / F::cast_from(2.0_f64) - t43930 + F::cast_from(112.0_f64) / F::cast_from(27.0_f64) * t43933 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43936 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43940;
    t44128
}
