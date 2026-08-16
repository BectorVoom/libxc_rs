//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1202/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1202(t41385: f64, t5587: f64, t16673: f64, t2629: f64, t2696: f64, t118: f64, t2375: f64, t5522: f64, t16710: f64, t2663: f64, t2517: f64, t2658: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58809 = t41385 * t5587;
    let t58811 = t16673 * t2629;
    let t58844 = t16673 * t2696;
    let t58972 = t5522 * t118 * t2375;
    let t58984 = t16710 * t2663;
    let t59013 = t2658 * t2517 * t5392;
    (t58809, t58811, t58844, t58972, t58984, t59013)
}
