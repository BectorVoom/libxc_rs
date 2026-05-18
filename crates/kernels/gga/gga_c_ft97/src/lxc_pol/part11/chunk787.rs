//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 787/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk787<F: Float>(t10286: F, t10243: F, t2755: F, t2789: F, t856: F, t91: F, t10397: F, t10251: F, t10255: F, t10404: F, t10407: F, t10412: F, t10417: F, t10420: F, t10424: F, t10428: F) -> (F, F) {
    let t10643 = F::new(2.0) / F::new(27.0) * t10286;
    let t10649 = F::new(2.0) / F::new(9.0) * t10243;
    let t10656 = t91 * t2755 * t856 * t2789;
    let t10658 = F::new(28.0) / F::new(81.0) * t10397;
    let t10659 = t10643 + F::new(2.0) / F::new(3.0) * t10407 + F::new(2.0) / F::new(9.0) * t10412 - F::new(2.0) / F::new(9.0) * t10420 + t10424 / F::new(3.0) + t10428 / F::new(3.0) - t10649 - F::new(2.0) / F::new(3.0) * t10251 - F::new(2.0) / F::new(3.0) * t10255 - F::new(2.0) / F::new(3.0) * t10404 + F::new(4.0) / F::new(9.0) * t10417 - t10656 / F::new(4.0) - t10658;
    (t10656, t10659)
}
