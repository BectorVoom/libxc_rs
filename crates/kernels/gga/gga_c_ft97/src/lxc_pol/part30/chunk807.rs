//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 807/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk807<F: Float>(t34111: F, t34115: F, t34120: F, t34124: F, t34126: F, t34130: F, t34133: F, t34136: F, t34139: F, t34142: F, t34146: F, t34150: F, t446: F) -> F {
    let t34153 = F::new(2.0) / F::new(3.0) * t446 * t34111 + F::new(4.0) / F::new(3.0) * t446 * t34115 + F::new(2.0) / F::new(3.0) * t446 * t34120 + t34124 - F::new(2.0) / F::new(3.0) * t446 * t34126 + F::new(2.0) / F::new(3.0) * t446 * t34130 - t446 * t34133 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t34136 - F::new(2.0) / F::new(3.0) * t446 * t34139 - t446 * t34142 / F::new(3.0) - t446 * t34146 / F::new(3.0) - t446 * t34150 / F::new(3.0);
    t34153
}
