//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1126/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1126(t40797: f64, t11050: f64, t8358: f64, t11885: f64, t6654: f64, t1010: f64, t37040: f64, t19155: f64, t11880: f64, t502: f64, t826: f64, t11033: f64, t2391: f64, param_eta: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40798 = 4.0_f64 / 3.0_f64 * t40797;
    let t40804 = t8358 * t11050;
    let t40805 = 4.0_f64 / 3.0_f64 * t40804;
    let t40806 = t6654 * t11885;
    let t40807 = 4.0_f64 / 3.0_f64 * t40806;
    let t40808 = t37040 * t1010;
    let t40815 = t19155 * param_eta;
    let t40821 = t11880 * t502 * t1010 * t826;
    let t40822 = 4.0_f64 * t40821;
    let t40840 = t11033 * t2391;
    (t40798, t40805, t40807, t40808, t40815, t40822, t40840)
}
