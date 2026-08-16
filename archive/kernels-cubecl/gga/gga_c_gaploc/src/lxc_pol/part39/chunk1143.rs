//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1143/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1143<F: Float>(t12255: F, t769: F, t3470: F, t313: F, t39403: F, t44085: F, t44089: F, t44092: F, t44093: F, t44097: F, t44099: F, t47486: F, t47488: F, t47492: F, t47494: F) -> F {
    let t47496 = t769 * t12255;
    let t47497 = t47496 * t3470;
    let t47500 = t313 * t39403;
    let t47501 = t47500 * t3470;
    let t47503 = -t44085 - t44089 - F::cast_from(0.79445533226334281487e-1_f64) * t47486 - F::cast_from(0.14896037479937677779e-1_f64) * t47488 - F::cast_from(0.14896037479937677779e-1_f64) * t47492 + F::cast_from(0.19171462976960374838e0_f64) * t47494 - F::cast_from(0.10725146985555128001e1_f64) * t47497 - t44092 - F::cast_from(0.69017266717057349418e1_f64) * t44093 - t44097 - t44099 - F::cast_from(0.10725146985555128001e1_f64) * t47501;
    t47503
}
