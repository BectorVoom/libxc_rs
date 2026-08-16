//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2329/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2329(t1362: f64, t19815: f64, t3799: f64, t6417: f64, t6422: f64, t1307: f64, t6330: f64, t12351: f64, t820: f64, t1799: f64, t5187: f64, t3870: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19904 = t19815 * t1362;
    let t19915 = t3799 * t6417;
    let t19917 = t3799 * t6422;
    let t19919 = t6330 * t1307;
    let t19921 = t12351 * t820 * t19919;
    let t19924 = t1799 * t5187;
    let t19926 = t3870 * t820 * t19924;
    (t19904, t19915, t19917, t19921, t19924, t19926)
}
