//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 733/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk733(t10660: f64, t10664: f64, t11352: f64, t11355: f64, t11358: f64, t11361: f64, t11382: f64, t1648: f64, t1815: f64, t4624: f64, t4652: f64, t4664: f64, t4667: f64, t574: f64) -> f64 {
    let t11385 = 3.0_f64 / 16.0_f64 * t11352 * t10664 - 3.0_f64 / 8.0_f64 * t11355 * t4624 - 3.0_f64 / 8.0_f64 * t4664 * t11358 + 3.0_f64 / 4.0_f64 * t11361 * t1648 + 3.0_f64 / 4.0_f64 * t4667 * t4652 + t1815 * t10660 / 4.0_f64 + t574 * t11382 / 2.0_f64;
    t11385
}
