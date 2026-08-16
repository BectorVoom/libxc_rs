//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2556/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2556(t1041: f64, t4584: f64, t49850: f64, t10422: f64, t14032: f64, t3070: f64, t13969: f64, t14166: f64, t14159: f64, t2960: f64, t14146: f64, t14068: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50047 = t1041 * t49850 * t4584;
    let t50056 = t3070 * t10422 * t14032;
    let t50062 = t1041 * t13969 * t14166;
    let t50077 = t2960 * t14159;
    let t50084 = t1041 * t13969 * t14146;
    let t50094 = t3070 * t10422 * t14068;
    (t50047, t50056, t50062, t50077, t50084, t50094)
}
