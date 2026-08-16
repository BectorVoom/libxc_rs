//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 731/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk731<F: Float>(t12661: F, t12665: F, t12667: F, t13054: F, t13057: F, t13059: F, t13859: F, t13863: F, t13867: F, t13874: F, t13878: F, t13882: F) -> F {
    let t14506 = -F::cast_from(0.76685851907841499353e0_f64) * t12661 + t13054 - t13057 - F::cast_from(0.76685851907841499352e0_f64) * t13059 - F::cast_from(0.92023022289409799224e1_f64) * t13859 + F::cast_from(0.23005755572352449806e2_f64) * t13863 - F::cast_from(0.13803453343411469884e2_f64) * t13867 + F::cast_from(0.59584149919750711115e-1_f64) * t12665 - F::cast_from(0.89376224879626066675e-1_f64) * t12667 - t13874 + t13878 + t13882;
    t14506
}
