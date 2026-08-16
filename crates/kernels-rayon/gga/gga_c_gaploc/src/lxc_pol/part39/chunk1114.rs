//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1114/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1114(t47243: f64, t7427: f64, t7573: f64, t43497: f64, t43500: f64, t43502: f64, t43511: f64, t43514: f64, t43516: f64, t43519: f64, t43523: f64, t43527: f64, t43529: f64, t43567: f64) -> f64 {
    let t47245 = t7427 * t7573 * t47243;
    let t47247 = -t43497 + t43500 + 0.14896037479937677779e-1_f64 * t43502 - t43511 + t43514 + 0.43710935587469654631e2_f64 * t43516 + 0.29792074959875355558e-1_f64 * t43519 + t43523 + t43527 - 0.14896037479937677779e-1_f64 * t43529 - 0.62115540045351614476e2_f64 * t47245 + t43567;
    t47247
}
