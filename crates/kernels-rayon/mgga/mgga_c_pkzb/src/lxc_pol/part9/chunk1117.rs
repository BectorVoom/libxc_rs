//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1117/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1117(t178: f64, t404: f64, t405: f64, t4902: f64, t2389: f64, t5939: f64, t918: f64, t2099: f64, t6516: f64, t6519: f64, t6525: f64, t6527: f64) -> (f64, f64, f64, f64) {
    let t19055 = 0.14820648238345094262e-3_f64 * t404 * t178 * t4902 * t405;
    let t19067 = t918 * t5939 * t2389;
    let t19070 = t6516 * t2099 * t6519;
    let t19073 = t6525 * t2099 * t6527;
    (t19055, t19067, t19070, t19073)
}
