//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1042/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1042(t7025: f64, t9231: f64, t6486: f64, t7032: f64, t240: f64, t67: f64, t1864: f64, t1860: f64, t6509: f64, t7031: f64, t2031: f64, t22489: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23975 = t9231 * t7025;
    let t23978 = t6486 * t7032;
    let t23992 = t240 * t67;
    let t23993 = t23992 * t1864;
    let t23995 = 88.0_f64 / 27.0_f64 * t1860 * t23993;
    let t23998 = t7031 * t6509;
    let t23999 = t1860 * t23998;
    let t24001 = t2031 * t22489;
    (t23975, t23978, t23992, t23993, t23995, t23998, t23999, t24001)
}
