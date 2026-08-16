//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1014/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1014(t11155: f64, t6088: f64, t7955: f64, t9782: f64, t352: f64, t6127: f64, t378: f64, t1196: f64, t3779: f64, t6290: f64, t1208: f64, t3806: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11157 = -t6088 + 0.71233333333333333332e-1_f64 * t7955 - 0.53424999999999999999e-1_f64 * t9782 + 0.53425e-1_f64 * t11155;
    let t11159 = 0.621814e-1_f64 * t11157 * t352;
    let t11163 = -t6127 + 0.37083333333333333334e-1_f64 * t7955 - 0.278125e-1_f64 * t9782 + 0.278125e-1_f64 * t11155;
    let t11164 = t11163 * t378;
    let t11166 = t3779 * t1196;
    let t11167 = t11166 * t6290;
    let t11180 = t3806 * t1208;
    (t11157, t11159, t11163, t11164, t11166, t11167, t11180)
}
