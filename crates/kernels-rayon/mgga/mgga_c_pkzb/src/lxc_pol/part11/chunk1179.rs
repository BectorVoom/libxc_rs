//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1179/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1179(t16615: f64, t16617: f64, t19754: f64, t10666: f64, t1769: f64, t10647: f64, t16416: f64, t10634: f64, t5381: f64, t10630: f64, t1727: f64, t2639: f64, t8914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28967 = 0.10389515463408878255e3_f64 * t16615;
    let t28968 = 0.10254018858216406658e4_f64 * t16617;
    let t28970 = 72.0_f64 * t19754;
    let t28977 = t1769 * t10666;
    let t28979 = t16416 * t10647;
    let t28990 = t5381 * t10634;
    let t28992 = t1727 * t10630;
    let t28995 = t8914 * t2639;
    (t28967, t28968, t28970, t28977, t28979, t28990, t28992, t28995)
}
