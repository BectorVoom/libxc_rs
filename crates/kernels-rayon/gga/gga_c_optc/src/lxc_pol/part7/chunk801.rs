//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 801/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk801(t7514: f64, t7517: f64, t7520: f64, t7529: f64, t7538: f64, t7544: f64, t7553: f64, t7555: f64, t7558: f64, t7560: f64, t7563: f64, t7566: f64, t7571: f64, t7573: f64) -> f64 {
    let t7575 = 0.19419375e1_f64 * t7514 - 0.3883875e1_f64 * t7517 + 0.247573125e0_f64 * t7520 + 0.16504875e0_f64 * t7553 + 0.258925e1_f64 * t7555 - 0.412621875e-1_f64 * t7558 - 0.33114e0_f64 * t7560 + 0.16557e0_f64 * t7563 - 0.49671e0_f64 * t7566 - 0.60385000000000000001e0_f64 * t7529 + 0.12077e1_f64 * t7538 - 0.181155e1_f64 * t7544 - 0.27595e0_f64 * t7571 + 0.16557e0_f64 * t7573;
    t7575
}
