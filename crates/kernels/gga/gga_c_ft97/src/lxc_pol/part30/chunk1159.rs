//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1159/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1159<F: Float>(t143187: F, t143204: F, t143245: F, t143497: F, t152948: F, t152952: F, t152954: F, t152958: F, t152962: F, t152965: F, t152970: F, t152975: F, t152979: F, t153375: F, t153379: F, t153384: F) -> F {
    let t154189 = F::new(24.0) * t152948 - F::new(12.0) * t152952 + F::new(2.0) / F::new(9.0) * t152954 - F::new(2.0) / F::new(3.0) * t152958 + t143187 / F::new(6.0) - F::new(8.0) / F::new(3.0) * t152962 - F::new(4.0) / F::new(3.0) * t152965 - F::new(2.0) / F::new(3.0) * t143204 - t143245 / F::new(3.0) - F::new(4.0) / F::new(3.0) * t152970 + F::new(2.0) * t152975 + F::new(2.0) * t152979 + t143497 - t153375 / F::new(2.0) - F::new(12.0) * t153379 + t153384 / F::new(3.0);
    t154189
}
