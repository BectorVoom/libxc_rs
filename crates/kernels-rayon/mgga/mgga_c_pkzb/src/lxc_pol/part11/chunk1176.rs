//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1176/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1176(t10731: f64, t1676: f64, t16506: f64, t19702: f64, t19710: f64, t23943: f64, t16532: f64, t1058: f64, t1535: f64, t16526: f64, t16531: f64, t16536: f64, t16539: f64, t16544: f64, t16548: f64, t24941: f64, t2536: f64, t8751: f64, t8758: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28922 = t10731 * t1676;
    let t28925 = 24.0_f64 * t16506;
    let t28928 = 0.31168546390226634765e3_f64 * t19702;
    let t28930 = 0.10526802520742363173e2_f64 * t19710;
    let t28931 = 3.0_f64 * t23943;
    let t28932 = 0.16265371950452609763e-1_f64 * t16532;
    let t28939 = -3.0_f64 * t1058 * t24941 * t2536 - 9.0_f64 * t1535 * t8751 * t8758 + t16526 + t16531 + t16536 - t16539 - t16544 + t16548 - t28930 + t28931 + t28932;
    (t28922, t28925, t28928, t28930, t28931, t28932, t28939)
}
