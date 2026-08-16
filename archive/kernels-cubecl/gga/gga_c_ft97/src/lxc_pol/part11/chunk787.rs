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
    let t10643 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10286;
    let t10649 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10243;
    let t10656 = t91 * t2755 * t856 * t2789;
    let t10658 = F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t10397;
    let t10659 = t10643 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10407 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10412 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10420 + t10424 / F::cast_from(3.0_f64) + t10428 / F::cast_from(3.0_f64) - t10649 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10251 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10255 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10404 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t10417 - t10656 / F::cast_from(4.0_f64) - t10658;
    (t10656, t10659)
}
