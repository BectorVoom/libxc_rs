//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 841/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk841(t11717: f64, t3503: f64, t11713: f64, t1210: f64, t11153: f64, t3439: f64, t11147: f64, t11545: f64, t3247: f64, t415: f64, t61: f64, t121: f64, t3584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11727 = t3503 * t11717;
    let t11728 = t11713 * t11727;
    let t11737 = t1210 * t11717;
    let t11738 = t11713 * t11737;
    let t11759 = t3439 * t11153;
    let t11764 = t11545 * t11147;
    let t11778 = 1.0_f64 / t415 / t3247;
    let t11779 = t61 * t11778;
    let t11784 = t121 * t3584;
    (t11727, t11728, t11737, t11738, t11759, t11764, t11778, t11779, t11784)
}
