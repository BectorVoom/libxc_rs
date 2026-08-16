//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2006/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2006(t27937: f64, t7032: f64, t1860: f64, t2031: f64, t2032: f64, t26028: f64, t26945: f64, t27979: f64, t28935: f64, t6486: f64, t7035: f64, t7428: f64, t7782: f64, t84285: f64, t92049: f64, t92056: f64, t96379: f64, t96383: f64, t96646: f64) -> f64 {
    let t102303 = t27937 * t7032;
    let t102305 = -2.0_f64 / 3.0_f64 * t96646 * t2032 - 2.0_f64 / 3.0_f64 * t27979 * t7035 + t6486 * t28935 / 3.0_f64 + t1860 * t2031 * t96379 / 3.0_f64 + 88.0_f64 / 27.0_f64 * t84285 - t92049 - t92056 + t96383 * t2032 / 3.0_f64 + t27937 * t7035 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t26028 * t7782 + 2.0_f64 / 3.0_f64 * t7428 * t26945 - 8.0_f64 / 9.0_f64 * t102303;
    t102305
}
