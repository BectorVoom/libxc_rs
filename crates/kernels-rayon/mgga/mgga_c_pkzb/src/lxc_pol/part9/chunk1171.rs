//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1171/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1171(t1020: f64, t1029: f64, t1031: f64, t133: f64, t158: f64, t162: f64, t17000: f64, t1742: f64, t20317: f64, t20318: f64, t20320: f64, t20321: f64, t20327: f64, t20344: f64, t20361: f64, t20380: f64, t2631: f64, t2632: f64, t2633: f64, t2636: f64, t5181: f64, t5217: f64, t5304: f64, t5348: f64, t5364: f64, t594: f64, t597: f64, t7055: f64, t7065: f64, t7070: f64, t7071: f64, t7081: f64) -> f64 {
    let t20397 = -36.0_f64 * t1742 * t133 * t2633 - 360.0_f64 * t2631 * t5304 * t1020 * t5181 + 180.0_f64 * t2631 * t7070 * t17000 + 180.0_f64 * t7065 * t7071 + 9.0_f64 * t594 * t7081 - (t20317 + t20318 + t20320 + t20321 + t20327 + t20344 + t20361 + t20380) * t158 * t162 - 12.0_f64 * t2631 * t2632 * t5217 + 9.0_f64 * t7055 * t597 + 9.0_f64 * t1742 * t2636 + 3.0_f64 * t1029 * t5364 + 3.0_f64 * t5348 * t1031;
    t20397
}
