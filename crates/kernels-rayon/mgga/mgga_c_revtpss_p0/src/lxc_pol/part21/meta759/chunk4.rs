//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2682/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2682(t2782: f64, t4086: f64, t49213: f64, t543: f64, t10136: f64, t14114: f64, t1882: f64, t2482: f64, t4104: f64, t4118: f64, t1892: f64, t9990: f64) -> (f64, f64, f64, f64) {
    let t49313 = t2782 * t4086 * t49213 * t543;
    let t49321 = t14114 * t10136;
    let t49322 = 0.39029762157531132076e-1_f64 * t49321;
    let t49325 = t2482 * t4118 * t1882 * t4104;
    let t49327 = t9990 * t1892;
    (t49313, t49322, t49325, t49327)
}
