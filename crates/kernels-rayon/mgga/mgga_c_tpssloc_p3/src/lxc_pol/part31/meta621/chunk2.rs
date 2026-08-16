//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1876/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1876(t26395: f64, t5187: f64, t6637: f64, t6888: f64, t22892: f64, t22893: f64, t28148: f64, t19761: f64, t1992: f64, t6976: f64, t1825: f64, t22633: f64, t90754: f64) -> (f64, f64, f64, f64) {
    let t97067 = t6888 * t6637 * t26395 * t5187;
    let t97070 = t22892 * t22893 * t28148;
    let t97079 = t1992 * t6976 * t19761;
    let t97083 = t22633 * t6976 * t90754 * t1825;
    (t97067, t97070, t97079, t97083)
}
