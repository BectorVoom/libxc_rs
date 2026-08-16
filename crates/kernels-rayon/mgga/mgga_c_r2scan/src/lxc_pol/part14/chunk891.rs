//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 891/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk891(t2148: f64, t8160: f64, t6165: f64, t1632: f64, t2531: f64, t551: f64, t574: f64, t2185: f64, t910: f64, t552: f64, t2654: f64, t1592: f64) -> (f64, f64, f64, f64) {
    let t8161 = t2148 * t8160;
    let t8163 = 0.34930954652346593434e-1_f64 * t6165 * t8161;
    let t8165 = t551 * t1632 * t2531;
    let t8167 = 0.23115257973478049502e0_f64 * t574 * t8165;
    let t8170 = t910 * t2185;
    let t8172 = t551 * t552 * t8170;
    let t8176 = t551 * t1632 * t2654;
    let t8178 = 0.69345773920434148506e0_f64 * t1592 * t8176;
    (t8163, t8167, t8172, t8178)
}
