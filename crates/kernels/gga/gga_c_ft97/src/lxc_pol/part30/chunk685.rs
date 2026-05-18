//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 685/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk685<F: Float>(t25036: F, t25042: F, t25146: F, t25154: F, t25163: F, t28811: F, t28814: F, t28819: F, t28824: F, t28829: F, t28833: F, t28838: F) -> F {
    let t28922 = -t25036 - t25042 / F::new(9.0) + t25146 / F::new(6.0) - t25154 - t28811 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t28814 + t28819 / F::new(4.0) + t28824 / F::new(4.0) - t25163 / F::new(18.0) + F::new(2.0) * t28829 + F::new(2.0) * t28833 + F::new(2.0) * t28838;
    t28922
}
