//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 712/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk712(t210: f64, t6795: f64, t6688: f64, t974: f64, t381: f64, t883: f64, t6743: f64, t6796: f64, t995: f64, t23602: f64, t3127: f64, t1011: f64, t3131: f64) -> (f64, f64, f64, f64, f64) {
    let t23631 = t6795 * t210;
    let t23632 = t974 * t6688;
    let t23633 = t23631 * t23632;
    let t23634 = t381 * t883;
    let t23635 = t6743 * t23634;
    let t23665 = t6796 * t995;
    let t23677 = t23602 * t3127;
    let t23678 = t1011 * t3131;
    (t23633, t23635, t23665, t23677, t23678)
}
