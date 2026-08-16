//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 426/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk426(t122: f64, t2096: f64, t39: f64, t500: f64, t162: f64, t9: f64, t267: f64, t57: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2097 = t2096 * t122;
    let t2098 = t39 * t500;
    let t2099 = t162 * t2098;
    let t2101 = 1.0_f64 / t9 / t2099;
    let t2102 = t2097 * t2101;
    let t2104 = t267 * t57;
    (t2097, t2098, t2099, t2101, t2102, t2104)
}
