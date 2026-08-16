//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1286/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1286(t174: f64, t5674: f64, t5079: f64, t944: f64, t3409: f64, t6400: f64, t1298: f64, t1841: f64, t3476: f64, t1131: f64, t1165: f64, t1531: f64, t1532: f64, t18436: f64, t18458: f64, t1899: f64, t301: f64, t335: f64, t3462: f64, t3463: f64, t367: f64, t372: f64, t406: f64, t4289: f64, t5746: f64, t5747: f64, t5922: f64, t6100: f64, t6288: f64, t839: f64, t929: f64, t960: f64) -> (f64, f64) {
    let t23804 = t174 * t5674;
    let t23821 = t944 * t5079;
    let t23831 = t3409 * t6400;
    let t23838 = t944 * t1298;
    let t23849 = t3476 * t1841;
    let t23852 = t335 * t960 * t23804 * t301 / 24.0_f64 + t367 * t960 * t6100 * t372 / 24.0_f64 + t367 * t960 * t1899 * t1131 / 48.0_f64 + t335 * t960 * t6288 * t839 / 48.0_f64 + 0.85748036236139473944e-3_f64 * t1531 * t1165 * t1532 * t23821 + 0.68598428988911579156e-2_f64 * t3462 * t1165 * t5922 * t3463 * t372 + 0.80031500487063509014e-2_f64 * t23831 - 0.34299214494455789578e-2_f64 * t18436 - 0.68598428988911579156e-2_f64 * t3462 * t1165 * t4289 * t5747 - 0.68598428988911579156e-2_f64 * t3462 * t1165 * t1532 * t23838 * t406 - 0.34299214494455789578e-2_f64 * t3462 * t1165 * t1532 * t5746 * t929 - 0.21437009059034868486e-3_f64 * t23849 + 0.16006300097412701803e-1_f64 * t18458;
    (t23821, t23852)
}
