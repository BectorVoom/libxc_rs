//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 768/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk768<F: Float>(t32113: F, t32331: F, t32348: F, t32118: F, t32123: F, t32328: F, t32336: F, t32341: F, t32345: F, t32353: F, t32358: F, t32362: F) -> (F, F, F, F) {
    let t32446 = t32113 / F::new(6.0);
    let t32449 = F::new(2.0) / F::new(3.0) * t32331;
    let t32453 = t32348 / F::new(3.0);
    let t32456 = t32446 + t32118 / F::new(6.0) + t32123 - t32328 / F::new(2.0) - t32449 - F::new(2.0) / F::new(3.0) * t32336 - F::new(6.0) * t32341 + F::new(4.0) * t32345 + t32453 + t32353 / F::new(3.0) + F::new(2.0) * t32358 - t32362;
    (t32446, t32449, t32453, t32456)
}
