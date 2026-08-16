//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 899/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk899(t900: f64, t9762: f64, t3033: f64, t3070: f64, t3740: f64, t6317: f64, t2192: f64, t3766: f64, t3743: f64, t6149: f64, t836: f64, t3041: f64, t3046: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9764 = 0.5848223622634646207e0_f64 * t9762 * t900;
    let t9766 = 2.0_f64 * t3033 * t3070;
    let t9768 = 2.0_f64 * t6317 * t3740;
    let t9770 = 1.0_f64 * t2192 * t3766;
    let t9771 = t6149 * t3743;
    let t9772 = t9771 * t836;
    let t9774 = t3041 * t3046;
    (t9764, t9766, t9768, t9770, t9771, t9772, t9774)
}
