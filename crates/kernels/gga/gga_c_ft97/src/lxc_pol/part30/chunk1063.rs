//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1063/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1063<F: Float>(t150288: F, t150291: F, t150295: F, t150298: F, t150302: F, t150304: F, t150308: F, t150915: F, t150918: F, t150922: F, t150927: F, t150931: F, t150935: F, t150939: F, t150943: F, t150946: F) -> F {
    let t151312 = t150288 / F::new(2.0) + t150291 / F::new(9.0) - F::new(2.0) / F::new(3.0) * t150295 + F::new(2.0) / F::new(3.0) * t150298 + F::new(4.0) / F::new(3.0) * t150302 - F::new(4.0) / F::new(27.0) * t150304 + t150308 - t150915 / F::new(3.0) - F::new(2.0) * t150918 + F::new(2.0) / F::new(3.0) * t150922 + t150927 / F::new(3.0) + t150931 / F::new(3.0) + t150935 / F::new(12.0) + t150939 / F::new(2.0) - t150943 / F::new(3.0) + t150946 / F::new(4.0);
    t151312
}
