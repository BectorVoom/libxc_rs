//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1143/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1143(t12255: f64, t769: f64, t3470: f64, t313: f64, t39403: f64, t44085: f64, t44089: f64, t44092: f64, t44093: f64, t44097: f64, t44099: f64, t47486: f64, t47488: f64, t47492: f64, t47494: f64) -> f64 {
    let t47496 = t769 * t12255;
    let t47497 = t47496 * t3470;
    let t47500 = t313 * t39403;
    let t47501 = t47500 * t3470;
    let t47503 = -t44085 - t44089 - 0.79445533226334281487e-1_f64 * t47486 - 0.14896037479937677779e-1_f64 * t47488 - 0.14896037479937677779e-1_f64 * t47492 + 0.19171462976960374838e0_f64 * t47494 - 0.10725146985555128001e1_f64 * t47497 - t44092 - 0.69017266717057349418e1_f64 * t44093 - t44097 - t44099 - 0.10725146985555128001e1_f64 * t47501;
    t47503
}
