//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 757/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk757(t2019: f64, t35604: f64, t640: f64, t7764: f64, t1343: f64, t2084: f64, t7765: f64, t1330: f64, t28: f64, t271: f64, t7553: f64, t7557: f64) -> (f64, f64, f64) {
    let t35607 = t2019 * t7764 * t640 * t35604;
    let t35608 = 0.45731474687362542471e-3_f64 * t35607;
    let t35611 = t2019 * t2084 * t1343 * t7765;
    let t35612 = 0.24390119833260022651e-2_f64 * t35611;
    let t35613 = t28 * t1330;
    let t35616 = t7553 * t35613 * t271 * t7557;
    (t35608, t35612, t35616)
}
