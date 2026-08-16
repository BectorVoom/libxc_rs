//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 496/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk496(t374: f64, t376: f64, t677: f64, t370: f64, t1032: f64, t1036: f64, t121: f64, t1023: f64, t248: f64, t1020: f64, t1017: f64, t1030: f64) -> (f64, f64, f64, f64, f64) {
    let t3082 = t374 * t677 * t376;
    let t3084 = t370 * t3082 / 13824.0_f64;
    let t3092 = t1032 * t1036;
    let t3101 = t121 * t376;
    let t3103 = t248 * t3101 * t1023;
    let t3104 = t1020 * t3103;
    let t3107 = t1030 * t1017;
    (t3084, t3092, t3101, t3104, t3107)
}
