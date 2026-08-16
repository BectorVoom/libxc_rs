//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1979/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1979(t1458: f64, t7408: f64, t2113: f64, t671: f64, t24932: f64, t26109: f64, t26111: f64, t26113: f64, t26116: f64, t26119: f64, t26121: f64, t26123: f64, t26125: f64, t26137: f64, t27371: f64, t27863: f64, t4072: f64, t7266: f64) -> (f64, f64, f64) {
    let t27879 = t7408 * t1458;
    let t27888 = t2113 * t671;
    let t27903 = 2.0_f64 * t1458 * t24932 + 2.0_f64 * t1458 * t27888 + 2.0_f64 * t27863 * t671 + 2.0_f64 * t4072 * t7266 + t26109 + t26111 + t26113 + t26116 + t26119 + t26121 + t26123 + t26125 + t26137 + t27371;
    (t27879, t27888, t27903)
}
