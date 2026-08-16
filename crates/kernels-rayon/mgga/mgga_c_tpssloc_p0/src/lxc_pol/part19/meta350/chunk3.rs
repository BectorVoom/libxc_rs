//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1275/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1275(t2794: f64, t2836: f64, t2842: f64, t2784: f64, t2791: f64, t2897: f64, t2929: f64, t10629: f64, t938: f64, t2903: f64, t2928: f64, t315: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41804 = 36.0_f64 * t2842 * t2794 * t2836;
    let t41811 = t2784 * t2791;
    let t41813 = 12.0_f64 * t41811 * t2794;
    let t41816 = t2897 * t2929;
    let t41821 = t938 * t10629;
    let t41825 = 1.0_f64 / t2928 / t2903;
    let t41826 = t315 * t41825;
    (t41804, t41813, t41816, t41821, t41825, t41826)
}
