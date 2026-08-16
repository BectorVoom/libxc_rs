//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1947/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1947(t101172: f64, t101176: f64, t101182: f64, t101187: f64, t101190: f64, t101193: f64, t101357: f64, t2048: f64, t26187: f64, t28105: f64, t28109: f64, t28112: f64, t7343: f64, t7352: f64, t7706: f64, t95255: f64, t95259: f64) -> f64 {
    let t101849 = 176.0_f64 / 27.0_f64 * t95255 - 2.0_f64 / 3.0_f64 * t101357 * t2048 - 5.0_f64 / 3.0_f64 * t95259 * t7706 - 10.0_f64 / 3.0_f64 * t26187 * t28105 - 10.0_f64 / 3.0_f64 * t26187 * t28109 - 5.0_f64 / 3.0_f64 * t7343 * t101172 - 10.0_f64 / 3.0_f64 * t7343 * t101176 - 5.0_f64 / 3.0_f64 * t7343 * t101182 - 2.0_f64 / 3.0_f64 * t101187 * t2048 - 4.0_f64 / 3.0_f64 * t101190 * t2048 - 4.0_f64 / 3.0_f64 * t101193 * t2048 - 4.0_f64 / 3.0_f64 * t28112 * t7352;
    t101849
}
