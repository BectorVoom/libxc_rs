//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1116/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1116(t10903: f64, t11764: f64, t2207: f64, t261: f64, t3299: f64, t7390: f64, t10879: f64, t11727: f64, t3304: f64, t7309: f64, t10740: f64, t980: f64) -> (f64, f64, f64, f64, f64) {
    let t40162 = t2207 * t10903 * t11764;
    let t40175 = t3299 * t261 * t7390;
    let t40177 = t10879 * t11727;
    let t40180 = t3304 * t261 * t7309;
    let t40185 = t980 * t10740;
    (t40162, t40175, t40177, t40180, t40185)
}
