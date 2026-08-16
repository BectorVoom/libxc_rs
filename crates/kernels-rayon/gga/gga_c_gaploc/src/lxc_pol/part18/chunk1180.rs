//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1180/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1180(t31754: f64, t19532: f64, t25723: f64, t9074: f64, t10163: f64, t1358: f64, t10172: f64, t1079: f64, t123: f64, t1367: f64, t30003: f64, t30005: f64, t30009: f64, t30014: f64, t30049: f64, t31731: f64, t31737: f64, t31740: f64, t31748: f64, t3359: f64, t3808: f64, t488: f64, t6507: f64) -> f64 {
    let t31755 = 0.142275033178380748e-1_f64 * t31754;
    let t31757 = t9074 * t19532 * t25723;
    let t31758 = 0.71137516589190373998e-2_f64 * t31757;
    let t31759 = t1358 * t10163;
    let t31760 = 0.31616674039640166222e-2_f64 * t31759;
    let t31761 = t30003 - t30005 - t30009 - t30014 + 0.18970004423784099732e-1_f64 * t1358 * t31731 * t1367 - t31737 - 0.63233348079280332442e-2_f64 * t3808 * t10172 - 0.63233348079280332442e-2_f64 * t1358 * t31740 * t123 * t488 - 0.12646669615856066488e-1_f64 * t1079 * t3359 - 0.12646669615856066488e-1_f64 * t1358 * t6507 * t31748 - t30049 + t31755 - t31758 - t31760;
    t31761
}
