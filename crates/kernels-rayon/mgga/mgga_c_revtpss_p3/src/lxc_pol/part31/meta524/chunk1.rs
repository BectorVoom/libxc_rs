//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1892/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1892(t1873: f64, t26004: f64, t5690: f64, t7252: f64, t25970: f64, t25976: f64, t26013: f64, t26015: f64, t27933: f64, t27937: f64, t27941: f64, t27943: f64, t27945: f64, t27947: f64, t27949: f64, t27951: f64, t27953: f64) -> (f64, f64) {
    let t27955 = t26004 * t1873;
    let t27957 = t7252 * t5690;
    let t27959 = t27933 / 16.0_f64 - t25970 + t25976 + 0.57165357490759649296e-4_f64 * t26015 + 0.57165357490759649296e-4_f64 * t27937 + t26013 + 0.85748036236139473944e-3_f64 * t27941 + 0.17149607247227894789e-2_f64 * t27943 - 0.42874018118069736972e-3_f64 * t27945 + 0.17149607247227894789e-2_f64 * t27947 - 0.17149607247227894789e-2_f64 * t27949 - 0.42874018118069736972e-3_f64 * t27951 - 0.25410001404642664113e-4_f64 * t27953 + 7.0_f64 / 144.0_f64 * t27955 - t27957 / 48.0_f64;
    (t27955, t27959)
}
