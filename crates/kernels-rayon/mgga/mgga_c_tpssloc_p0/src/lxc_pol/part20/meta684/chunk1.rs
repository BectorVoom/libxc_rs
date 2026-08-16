//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2593/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2593(t1215: f64, t2244: f64, t475: f64, t3242: f64, t1216: f64, t3493: f64, t1011: f64, t1212: f64, t52446: f64, t11539: f64, t1174: f64, t14736: f64) -> (f64, f64, f64, f64, f64) {
    let t52537 = t2244 * t1215;
    let t52538 = t52537 * t475;
    let t52548 = t475 * t3242;
    let t52554 = t1216 * t3493;
    let t52568 = t52446 * t1011 * t1212;
    let t52575 = t1174 * t11539 * t14736;
    (t52538, t52548, t52554, t52568, t52575)
}
