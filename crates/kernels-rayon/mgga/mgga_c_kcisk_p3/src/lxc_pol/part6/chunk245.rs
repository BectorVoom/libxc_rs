//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 245/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk245(t1007: f64, t1054: f64, t1136: f64, t1140: f64, t1147: f64, t289: f64, t298: f64, t301: f64, t430: f64) -> (f64, f64) {
    let t1149 = t1136 * t289 - t1140 * t1147 - t1007 + t1054;
    let t1161 = t298 * t430 * t301;
    (t1149, t1161)
}
