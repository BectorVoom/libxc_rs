//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1324/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1324(t16869: f64, t16910: f64, t16979: f64, t17020: f64, t235: f64, t5631: f64, t814: f64, t829: f64, t252: f64, t5611: f64, t4182: f64, t1499: f64, t4280: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17022 = t16869 + t16910 + t16979 + t17020;
    let t17023 = t235 * t17022;
    let t17027 = t814 * t5631;
    let t17028 = t17027 * t829;
    let t17030 = t252 * t5611;
    let t17031 = t17030 * t4182;
    let t17034 = t1499 * t4280;
    (t17022, t17023, t17028, t17030, t17031, t17034)
}
