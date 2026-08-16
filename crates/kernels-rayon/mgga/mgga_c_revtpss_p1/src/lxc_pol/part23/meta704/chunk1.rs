//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2455/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2455(t1389: f64, t3964: f64, t40604: f64, t10111: f64, t22: f64, t4092: f64, t39515: f64, t4083: f64, t10043: f64, t9303: f64, t14192: f64, t555: f64) -> (f64, f64, f64, f64, f64) {
    let t47337 = 0.11344944493805280483e-2_f64 * t3964 * t40604 * t1389;
    let t47348 = t10111 * t4092 * t22;
    let t47351 = 0.11564373972601816912e-1_f64 * t39515 * t4083;
    let t47352 = t9303 * t10043;
    let t47371 = t14192 * t555;
    (t47337, t47348, t47351, t47352, t47371)
}
