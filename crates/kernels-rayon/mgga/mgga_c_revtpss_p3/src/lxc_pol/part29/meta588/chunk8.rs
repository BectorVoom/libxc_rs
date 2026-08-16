//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1949/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1949(t28112: f64, t7349: f64, t28116: f64, t28119: f64, t26169: f64, t7709: f64, t60221: f64, t7342: f64, t6960: f64, t95268: f64, t95270: f64, t95284: f64, t95286: f64, t95288: f64, t95290: f64, t95294: f64) -> f64 {
    let t101879 = 32.0_f64 / 9.0_f64 * t28112 * t7349;
    let t101881 = 32.0_f64 / 9.0_f64 * t28116 * t7349;
    let t101883 = 32.0_f64 / 9.0_f64 * t28119 * t7349;
    let t101885 = 32.0_f64 / 9.0_f64 * t7709 * t26169;
    let t101886 = t60221 * t7342;
    let t101896 = t101879 + t101881 + t101883 + t101885 - 10.0_f64 / 3.0_f64 * t101886 * t6960 + 80.0_f64 / 9.0_f64 * t95268 + 32.0_f64 / 9.0_f64 * t95270 + 80.0_f64 / 9.0_f64 * t95284 + 40.0_f64 / 9.0_f64 * t95286 + 32.0_f64 / 9.0_f64 * t95288 + 16.0_f64 / 9.0_f64 * t95290 - 880.0_f64 / 27.0_f64 * t95294;
    t101896
}
