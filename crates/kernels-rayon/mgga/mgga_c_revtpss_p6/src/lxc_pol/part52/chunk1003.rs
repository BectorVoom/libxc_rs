//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1003/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1003(t27937: f64, t27955: f64, t26016: f64, t26310: f64, t26312: f64, t26325: f64, t27933: f64, t27941: f64, t27943: f64, t27945: f64, t27947: f64, t27949: f64, t27951: f64, t27953: f64, t27957: f64) -> f64 {
    let t28877 = 0.11433071498151929859e-3_f64 * t27937;
    let t28885 = 7.0_f64 / 72.0_f64 * t27955;
    let t28887 = t27933 / 8.0_f64 - t26310 + t26312 + t26016 + t28877 + t26325 + 0.17149607247227894789e-2_f64 * t27941 + 0.34299214494455789578e-2_f64 * t27943 - 0.85748036236139473944e-3_f64 * t27945 + 0.34299214494455789578e-2_f64 * t27947 - 0.34299214494455789578e-2_f64 * t27949 - 0.85748036236139473944e-3_f64 * t27951 - 0.50820002809285328225e-4_f64 * t27953 + t28885 - t27957 / 24.0_f64;
    t28887
}
