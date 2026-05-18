//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 822/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk822<F: Float>(t33105: F, t33118: F, t143: F, t160: F, t1359: F, t5968: F, t574: F, t605: F, t1901: F, t28: F, t33052: F, t33057: F, t33062: F, t33066: F, t33068: F, t33072: F, t33077: F, t33082: F, t33087: F, t33092: F, t446: F, t89: F) -> (F, F, F, F, F) {
    let t33119 = t33105 + t33118;
    let t33121 = t143 * t33119 * t160;
    let t33125 = t1359 * t5968;
    let t33127 = t574 * t605 * t33125;
    let t33130 = t446 * t33052 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t1901 * t33057 + F::new(2.0) / F::new(3.0) * t446 * t33062 - t33066 - F::new(2.0) / F::new(9.0) * t1901 * t33068 + F::new(2.0) / F::new(3.0) * t446 * t33072 + t446 * t33077 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t446 * t33082 - F::new(2.0) * t446 * t33087 + F::new(4.0) / F::new(3.0) * t446 * t33092 + t89 * t28 * t33121 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t33127;
    (t33119, t33121, t33125, t33127, t33130)
}
