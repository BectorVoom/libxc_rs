//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 872/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk872(t11352: f64, t16037: f64, t1815: f64, t22599: f64, t2372: f64, t28393: f64, t28414: f64, t28546: f64, t28568: f64, t4664: f64, t574: f64, t6774: f64, t8504: f64, t8522: f64) -> f64 {
    let t28571 = 3.0_f64 / 16.0_f64 * t11352 * t28414 - 3.0_f64 / 8.0_f64 * t16037 * t8504 - 3.0_f64 / 8.0_f64 * t4664 * t28546 + 3.0_f64 / 4.0_f64 * t22599 * t2372 + 3.0_f64 / 4.0_f64 * t6774 * t8522 + t1815 * t28393 / 4.0_f64 + t574 * t28568 / 2.0_f64;
    t28571
}
