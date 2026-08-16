//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2098/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2098(t372: f64, t4801: f64, t4181: f64, t4786: f64, t1062: f64, t4857: f64) -> (f64, f64, f64, f64) {
    let t15701 = t372 * t4801;
    let t15702 = t4181 * t4786;
    let t15703 = t15701 * t15702;
    let t15707 = t4857 * t1062;
    (t15701, t15702, t15703, t15707)
}
