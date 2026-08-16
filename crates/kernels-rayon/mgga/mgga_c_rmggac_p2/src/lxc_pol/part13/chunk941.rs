//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 941/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk941(t2320: f64, t36520: f64, t2310: f64, t7921: f64, t2289: f64, t35277: f64, t9005: f64, t9128: f64, t4895: f64, t645: f64, t1550: f64, t11905: f64, t2061: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40679 = t36520 * t2320;
    let t40681 = t7921 * t2310;
    let t40683 = t35277 * t2289;
    let t40685 = t9128 * t9005;
    let t40687 = t645 * t4895;
    let t40688 = t1550 * t40687;
    let t40690 = t11905 * t2061;
    (t40679, t40681, t40683, t40685, t40687, t40688, t40690)
}
