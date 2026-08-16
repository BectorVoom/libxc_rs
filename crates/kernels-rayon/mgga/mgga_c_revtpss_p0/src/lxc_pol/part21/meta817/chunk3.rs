//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3008/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3008(t1062: f64, t43154: f64, t11202: f64, t1651: f64, t11940: f64, t3105: f64, t11923: f64, t15926: f64, t11922: f64, t16016: f64, t4899: f64, t11994: f64, t15734: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54982 = t43154 * t1062;
    let t54983 = t1651 * t11202;
    let t54988 = t11940 * t3105;
    let t54991 = t15926 * t11923;
    let t54994 = t4899 * t11922 * t16016;
    let t55000 = t11994 * t15734;
    (t54982, t54983, t54988, t54991, t54994, t55000)
}
