//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 812/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk812(t7514: f64, t7517: f64, t7520: f64, t7529: f64, t7538: f64, t7544: f64, t7553: f64, t7555: f64, t7558: f64, t7560: f64, t7563: f64, t7566: f64, t7571: f64, t7573: f64) -> f64 {
    let t7647 = 0.142419375e1_f64 * t7514 - 0.28483875e1_f64 * t7517 + 0.46074375e0_f64 * t7520 + 0.3071625e0_f64 * t7553 + 0.1898925e1_f64 * t7555 - 0.76790625e-1_f64 * t7558 - 0.32862666666666666666e0_f64 * t7560 + 0.16431333333333333333e0_f64 * t7563 - 0.49293999999999999999e0_f64 * t7566 - 0.59793333333333333333e0_f64 * t7529 + 0.11958666666666666667e1_f64 * t7538 - 0.17938e1_f64 * t7544 - 0.27385555555555555556e0_f64 * t7571 + 0.16431333333333333333e0_f64 * t7573;
    t7647
}
