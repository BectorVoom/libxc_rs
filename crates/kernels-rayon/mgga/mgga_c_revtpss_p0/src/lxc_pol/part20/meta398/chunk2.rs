//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1475/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1475(t11452: f64, t2962: f64, t41306: f64, t41308: f64, t41312: f64, t41316: f64, t41320: f64, t41323: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64) -> (f64, f64) {
    let t41895 = t2962 * t11452;
    let t41908 = 0.17757530864197530864e0_f64 * t41306;
    let t41913 = 0.13698666666666666667e0_f64 * t41308 + 0.41096e0_f64 * t41312 - 0.61644e0_f64 * t41316 + 0.10274e0_f64 * t41320 + 0.41095999999999999998e0_f64 * t41323 - 0.34246666666666666665e-1_f64 * t41327 + t41908 - 0.45662222222222222221e-1_f64 * t41330 - 0.3044148148148148148e-1_f64 * t41332 + 0.22831111111111111111e-1_f64 * t41334 + 0.25367901234567901233e-1_f64 * t41336;
    (t41895, t41913)
}
