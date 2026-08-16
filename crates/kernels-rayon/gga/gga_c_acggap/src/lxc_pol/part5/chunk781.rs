//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 781/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk781(t5034: f64, t1708: f64, t75: f64, t288: f64, t1: f64, t283: f64, t2996: f64, t2998: f64, t3000: f64, t5040: f64, t5045: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6020 = 24.0_f64 * t5034;
    let t6021 = t1708 * t75;
    let t6022 = t6021 * t288;
    let t6023 = 0.5848223622634646207e0_f64 * t6022;
    let t6024 = t1708 * t1;
    let t6025 = t6024 * t283;
    let t6026 = 0.18311447306006545054e-3_f64 * t6025;
    let t6027 = 32.0_f64 * t2996;
    let t6028 = 20.0_f64 * t2998;
    let t6029 = 8.0_f64 * t3000;
    let t6030 = 0.34631718211362927517e2_f64 * t5040;
    let t6031 = 0.11696447245269292414e1_f64 * t5045;
    (t6020, t6021, t6022, t6023, t6024, t6025, t6026, t6027, t6028, t6029, t6030, t6031)
}
