//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 708/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk708<F: Float>(t13212: F, t27073: F, t1901: F, t23468: F, t23484: F, t27199: F, t27203: F, t27205: F, t27208: F, t27212: F, t27217: F, t27222: F, t27226: F, t27229: F, t27232: F, t446: F) -> F {
    let t27235 = t13212 * t27073;
    let t27238 = -t446 * t27199 / F::new(3.0) - t23468 / F::new(27.0) + t27203 / F::new(9.0) + t27205 / F::new(9.0) - t1901 * t27208 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t27212 - F::new(2.0) / F::new(9.0) * t1901 * t27217 + F::new(2.0) / F::new(27.0) * t1901 * t27222 + t23484 / F::new(9.0) + t27226 / F::new(9.0) - t1901 * t27229 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t27232 + F::new(2.0) / F::new(27.0) * t1901 * t27235;
    t27238
}
