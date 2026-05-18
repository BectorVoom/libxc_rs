//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 314/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk314<F: Float>(t2681: F, t4218: F, t824: F, t192: F, t4129: F, t852: F, t2761: F, t2762: F, t2764: F, t3139: F, t4197: F, t4200: F, t4203: F, t4207: F, t4210: F, t4213: F, t4215: F, t462: F, t92: F) -> F {
    let t4220 = t2681 * t4218 * t824;
    let t4224 = t192 * t852 * t4129;
    let t4226 = t2761 + t2762 / F::new(9.0) + t2764 / F::new(3.0) + t4197 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t462 * t4200 + t462 * t4203 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t462 * t4207 + F::new(2.0) / F::new(3.0) * t3139 * t4210 + t4213 / F::new(3.0) + t462 * t4215 / F::new(3.0) + F::new(2.0) * t462 * t4220 - t92 * t4224;
    t4226
}
