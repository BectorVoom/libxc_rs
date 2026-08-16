//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1206/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1206(t11342: f64, t43726: f64, t11555: f64, t12098: f64, t3275: f64, t11486: f64, t3262: f64, t11506: f64, t41337: f64, t3579: f64, t41816: f64, t12811: f64, t1561: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44029 = 3.0_f64 / 4.0_f64 * t43726 * t11342;
    let t44032 = 5.0_f64 / 8.0_f64 * t3275 * t12098 * t11555;
    let t44035 = 15.0_f64 / 8.0_f64 * t3262 * t12098 * t11486;
    let t44037 = 3.0_f64 / 2.0_f64 * t11506 * t41337;
    let t44039 = 5.0_f64 / 8.0_f64 * t3579 * t41816;
    let t44040 = t1561 * t12811;
    (t44029, t44032, t44035, t44037, t44039, t44040)
}
