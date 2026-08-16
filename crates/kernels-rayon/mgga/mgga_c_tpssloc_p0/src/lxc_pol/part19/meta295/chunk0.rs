//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1076/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1076(t649: f64, t671: f64, t157: f64, t9929: f64, t2379: f64, t262: f64, t9897: f64, t2570: f64, t67: f64, t792: f64, t131: f64, t9558: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12734 = t649 * t671;
    let t12908 = t9929 * t157;
    let t12935 = t2379 * t262;
    let t12939 = t9897 * t157;
    let t12997 = t2570 * t67;
    let t12998 = t792 * t12997;
    let t13004 = t9558 * t131;
    (t12734, t12908, t12935, t12939, t12998, t13004)
}
