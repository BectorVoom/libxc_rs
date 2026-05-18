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
    let t44121 = F::new(280.0) / F::new(81.0) * t43537;
    let t44128 = -F::new(8.0) * t43511 + F::new(6.0) * t43516 + F::new(16.0) / F::new(3.0) * t43519 + F::new(8.0) * t43522 + F::new(24.0) * t43528 + F::new(4.0) / F::new(3.0) * t43531 - F::new(16.0) / F::new(27.0) * t43534 + t44121 - F::new(3.0) / F::new(4.0) * t43541 - F::new(15.0) / F::new(16.0) * t43551 + t43926 / F::new(2.0) - t43930 + F::new(112.0) / F::new(27.0) * t43933 - F::new(8.0) / F::new(3.0) * t43936 + F::new(8.0) / F::new(3.0) * t43940;
    t44128
}
