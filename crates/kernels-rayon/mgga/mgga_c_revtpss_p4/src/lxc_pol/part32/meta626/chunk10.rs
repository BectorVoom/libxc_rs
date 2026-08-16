//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2000/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2000(t101788: f64, t7706: f64, t29538: f64, t7349: f64, t101883: f64, t101885: f64, t108765: f64, t108816: f64, t2048: f64, t28112: f64, t28116: f64, t28119: f64, t28635: f64, t29554: f64, t7352: f64, t7709: f64, t7964: f64, t95294: f64) -> f64 {
    let t110008 = t101788 * t7706;
    let t110010 = t29538 * t7349;
    let t110012 = t101883 + t101885 - 440.0_f64 / 27.0_f64 * t95294 - 2.0_f64 / 3.0_f64 * t108765 * t2048 - 2.0_f64 / 3.0_f64 * t108816 * t2048 - 2.0_f64 / 3.0_f64 * t29554 * t7352 - 4.0_f64 / 3.0_f64 * t28112 * t7964 - 4.0_f64 / 3.0_f64 * t28116 * t7964 - 4.0_f64 / 3.0_f64 * t28119 * t7964 - 4.0_f64 / 3.0_f64 * t7709 * t28635 + 80.0_f64 / 9.0_f64 * t110008 + 32.0_f64 / 9.0_f64 * t110010;
    t110012
}
