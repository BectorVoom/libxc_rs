//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 920/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk920(t8762: f64, t8764: f64, t20: f64, t4879: f64, t62: f64, t879: f64, t2740: f64, t882: f64, t209: f64, t207: f64, t69: f64, t2739: f64, t6: f64) -> (f64, f64, f64, f64, f64) {
    let t8765 = t8762 * t8764;
    let t8769 = t62 * t4879 * t20;
    let t8778 = t879 * t879;
    let t8779 = 1.0_f64 / t8778;
    let t8780 = t2740 * t882;
    let t8782 = t209 * t8779 * t8780;
    let t8785 = t207 * t69;
    let t8786 = t6 * t2739;
    (t8765, t8769, t8782, t8785, t8786)
}
