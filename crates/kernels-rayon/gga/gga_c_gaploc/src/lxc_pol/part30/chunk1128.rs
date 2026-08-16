//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1128/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1128(t1349: f64, t9070: f64, t20237: f64, t2321: f64, t9074: f64, t23927: f64, t4255: f64, t883: f64, t9204: f64, t123: f64, t20008: f64, t6486: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30098 = 0.63233348079280332442e-2_f64 * t1349 * t9070;
    let t30103 = 0.23712505529730124666e-2_f64 * t9074 * t20237 * t2321;
    let t30105 = 0.47425011059460249332e-2_f64 * t23927 * t9070;
    let t30110 = t883 * t4255;
    let t30113 = 0.16598753870811087267e-1_f64 * t9074 * t9204 * t30110;
    let t30118 = 0.284550066356761496e-1_f64 * t9074 * t20008 * t123 * t6486;
    (t30098, t30103, t30105, t30110, t30113, t30118)
}
