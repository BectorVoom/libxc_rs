//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2327/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2327<F: Float>(t16153: F, t221: F, t26284: F, t26289: F, t6604: F, t80887: F, t91133: F, t91136: F, t91138: F, t91141: F, t91144: F, t91145: F, t91147: F, t91149: F, t91155: F, t91159: F, t91162: F, t91163: F, t91165: F, t91167: F, t91171: F, t91173: F) -> F {
    let t91176 = t26284 * t221 * t16153;
    let t91179 = t80887 * t6604 * t26289;
    let t91180 = F::cast_from(0.11869590291677274911e0_f64) * t91179;
    let t91181 = F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t91133 + t91136 + t91138 - t91141 - t91144 - t91145 / F::cast_from(192.0_f64) - t91147 / F::cast_from(384.0_f64) - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t91149 + t91155 - t91159 + t91162 - t91163 / F::cast_from(384.0_f64) - t91165 / F::cast_from(384.0_f64) - F::cast_from(0.11304371706359309439e-1_f64) * t91167 - t91171 + t91173 / F::cast_from(8.0_f64) + t91176 / F::cast_from(16.0_f64) - t91180;
    t91181
}
