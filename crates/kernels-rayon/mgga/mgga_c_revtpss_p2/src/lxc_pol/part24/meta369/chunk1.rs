//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1256/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1256(t1169: f64, t24330: f64, t1188: f64, t24375: f64, t12397: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64) -> (f64, f64, f64) {
    let t24431 = t24330 * t1169;
    let t24436 = t24375 * t1188;
    let t24453 = -t12397 + 0.2283111111111111111e-1_f64 * t16706 + 0.11415555555555555555e-1_f64 * t20283 - 0.34246666666666666665e-1_f64 * t20285 - 0.17123333333333333333e-1_f64 * t20287 + 0.19025925925925925925e-1_f64 * t24230 - 0.68493333333333333331e-1_f64 * t24234 - 0.34246666666666666665e-1_f64 * t24238 + 0.10274e0_f64 * t24242 + 0.10274e0_f64 * t24246 + 0.17123333333333333333e-1_f64 * t24250;
    (t24431, t24436, t24453)
}
