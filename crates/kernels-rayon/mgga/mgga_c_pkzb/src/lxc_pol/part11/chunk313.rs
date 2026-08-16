//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 313/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk313(t1068: f64, t1074: f64, t1077: f64, t1081: f64, t697: f64, t700: f64) -> f64 {
    let t1095 = 0.3529725e1_f64 * t1074 - t697 + 0.1549425e1_f64 * t1068 + 0.6311625e0_f64 * t1077 - t700 + 0.312585e0_f64 * t1081;
    t1095
}
