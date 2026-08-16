//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1014/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1014(t11673: f64, t11674: f64, t10357: f64, t128: f64, t2207: f64, t10350: f64, t3737: f64, t6940: f64, t2415: f64, t959: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11675 = t11673 * t11674;
    let t11676 = t11675 * t10357;
    let t11678 = t2207 * t128;
    let t11679 = t11673 * t11678;
    let t11680 = t11679 * t10350;
    let t11682 = t3737 * t6940;
    let t11683 = t2415 * t959;
    (t11675, t11676, t11678, t11679, t11680, t11682, t11683)
}
