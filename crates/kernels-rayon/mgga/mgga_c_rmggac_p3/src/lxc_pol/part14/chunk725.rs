//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 725/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk725(t7559: f64, t7562: f64, t7767: f64, t2181: f64, t7561: f64, t2165: f64, t638: f64, t7184: f64, t2169: f64, t1343: f64, t7321: f64, t1327: f64, t4765: f64, t640: f64, t7352: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34612 = 0.13010691197123848594e-3_f64 * t7559;
    let t34613 = 0.10000709273223291967e0_f64 * t7562;
    let t34649 = 0.91462949374725084942e-3_f64 * t7767;
    let t34659 = t2181 * t7561;
    let t34662 = t638 * t7184 * t2165;
    let t34665 = t638 * t7184 * t2169;
    let t34683 = t7321 * t1343;
    let t34687 = t4765 * t34683 * t640 * t7352 * t1327;
    (t34612, t34613, t34649, t34659, t34662, t34665, t34687)
}
