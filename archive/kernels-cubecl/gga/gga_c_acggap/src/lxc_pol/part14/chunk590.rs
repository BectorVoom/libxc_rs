//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 590/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk590<F: Float>(t50: F, t1702: F, t829: F, t1289: F, t238: F, t296: F, t5468: F, t5493: F, t822: F, t5492: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t5498 = t829 * t1702;
    let t5504 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5493 * t238 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1289 * t822 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5498 * t238 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t296 * t5468);
    let t5506 = t5492 / F::cast_from(2.0_f64) + t5504 / F::cast_from(2.0_f64);
    t5506
}
