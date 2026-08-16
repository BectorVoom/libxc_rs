//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1130/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1130(t1010: f64, t37040: f64, t19155: f64, t11880: f64, t502: f64, t826: f64, t37041: f64, t11033: f64, t2391: f64, t3366: f64, t8355: f64, t37066: f64, param_eta: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40808 = t37040 * t1010;
    let t40815 = t19155 * param_eta;
    let t40821 = t11880 * t502 * t1010 * t826;
    let t40839 = 22.0_f64 / 9.0_f64 * t37041;
    let t40840 = t11033 * t2391;
    let t40844 = t8355 * t3366;
    let t40846 = 22.0_f64 / 9.0_f64 * t37066;
    (t40808, t40815, t40821, t40839, t40840, t40844, t40846)
}
