//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3085/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3085<F: Float>(t43780: F, t43782: F, t43816: F, t43820: F, t50952: F, t50954: F, t63355: F, t63359: F, t63361: F, t63365: F, t63370: F, t63374: F) -> F {
    let t63980 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t50952 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t50954 + t43820 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t43780 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t43782 - F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t43816 + t63355 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t63359 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t63361 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t63365 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t63370 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t63374;
    t63980
}
