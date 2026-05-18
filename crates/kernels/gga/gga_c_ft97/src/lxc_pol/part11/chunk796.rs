//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 796/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk796<F: Float>(t10397: F, t10243: F, t10251: F, t10255: F, t10286: F, t10404: F, t10407: F, t10412: F, t10417: F, t10420: F, t10424: F, t10428: F, t10656: F) -> F {
    let t10797 = F::new(28.0) / F::new(27.0) * t10397;
    let t10798 = F::new(2.0) / F::new(9.0) * t10286 + F::new(2.0) * t10407 + F::new(2.0) / F::new(3.0) * t10412 - F::new(2.0) / F::new(3.0) * t10420 + t10424 + t10428 - F::new(2.0) / F::new(3.0) * t10243 - F::new(2.0) * t10251 - F::new(2.0) * t10255 - F::new(2.0) * t10404 + F::new(4.0) / F::new(3.0) * t10417 - F::new(3.0) / F::new(4.0) * t10656 - t10797;
    t10798
}
