//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1920/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1920(t1992: f64, t54840: f64, t550: f64, t6976: f64, t54883: f64, t22633: f64, t22897: f64, t26421: f64, t3793: f64, t16041: f64, t22704: f64, t5336: f64, t80798: f64) -> (f64, f64, f64, f64, f64) {
    let t90883 = t1992 * t6976 * t54840 * t550;
    let t90887 = t1992 * t6976 * t54883 * t550;
    let t90892 = t22633 * t22897 * t26421 * t3793;
    let t90895 = t1992 * t22897 * t16041;
    let t90898 = t22704 * t80798 * t5336;
    (t90883, t90887, t90892, t90895, t90898)
}
