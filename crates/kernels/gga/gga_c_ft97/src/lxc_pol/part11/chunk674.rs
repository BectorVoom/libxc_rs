//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 674/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk674<F: Float>(t8805: F, t9065: F, t9068: F, t8796: F, t8799: F, t8802: F, t9010: F, t9020: F, t9035: F, t9039: F, t9043: F, t9047: F, t9052: F) -> F {
    let t9366 = F::new(2.0) / F::new(3.0) * t8805;
    let t9369 = F::new(4.0) / F::new(9.0) * t9065;
    let t9370 = t9068 / F::new(3.0);
    let t9371 = F::new(4.0) / F::new(27.0) * t8796;
    let t9372 = t8799 / F::new(9.0);
    let t9373 = F::new(2.0) / F::new(27.0) * t8802;
    let t9379 = -t9366 - t9010 / F::new(3.0) - F::new(2.0) * t9020 - t9369 + t9370 - t9371 + t9372 + t9373 + F::new(2.0) / F::new(3.0) * t9035 - F::new(2.0) / F::new(9.0) * t9039 + t9043 / F::new(3.0) + t9047 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t9052;
    t9379
}
