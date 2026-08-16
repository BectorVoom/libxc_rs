//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1128/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1128(t1250: f64, t3101: f64, t1254: f64, t3106: f64, t6383: f64, t871: f64, t23927: f64, t9083: f64, t29874: f64, t9205: f64, t123: f64, t21004: f64, t2326: f64, t9074: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29913 = t3101 * t1250;
    let t29915 = t3106 * t1254;
    let t29923 = t6383 * t871;
    let t30003 = 0.47425011059460249332e-2_f64 * t23927 * t9083;
    let t30005 = 0.142275033178380748e-1_f64 * t29874 * t9205;
    let t30009 = 0.142275033178380748e-1_f64 * t9074 * t21004 * t123 * t2326;
    (t29913, t29915, t29923, t30003, t30005, t30009)
}
