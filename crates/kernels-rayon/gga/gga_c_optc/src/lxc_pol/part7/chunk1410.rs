//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1410/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1410(t11596: f64, t11597: f64, t26459: f64, t26463: f64, t26467: f64, t26470: f64, t26472: f64, t26476: f64, t26479: f64, t26482: f64, t26484: f64, t27346: f64, t28061: f64, t2908: f64, t3268: f64, t3980: f64, t4281: f64, t9254: f64) -> f64 {
    let t28063 = 0.31013857721884116596e-1_f64 * t3980 * t2908 * t9254 * t3268 + t26459 + 28.0_f64 / 9.0_f64 * t4281 * t11596 * t11597 * t27346 + t26463 + t26467 + 2.0_f64 / 3.0_f64 * t28061 - t26470 - t26472 + t26476 - t26479 - t26482 + t26484;
    t28063
}
