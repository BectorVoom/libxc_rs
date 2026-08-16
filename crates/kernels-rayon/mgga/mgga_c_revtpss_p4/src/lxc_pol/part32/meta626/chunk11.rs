//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2001/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2001(t26179: f64, t29544: f64, t29548: f64, t29554: f64, t7349: f64, t28640: f64, t7709: f64, t29562: f64, t95319: f64, t101899: f64, t101901: f64, t101903: f64, t101906: f64, t101907: f64, t101929: f64, t95314: f64) -> f64 {
    let t110014 = t26179 * t29544;
    let t110016 = t26179 * t29548;
    let t110018 = t29554 * t7349;
    let t110020 = t7709 * t28640;
    let t110022 = t95319 * t29562;
    let t110027 = 80.0_f64 / 9.0_f64 * t110014 + 40.0_f64 / 9.0_f64 * t110016 + 16.0_f64 / 9.0_f64 * t110018 + 32.0_f64 / 9.0_f64 * t110020 - 80.0_f64 / 3.0_f64 * t110022 - t101899 - t101901 - t101903 - t101906 + 176.0_f64 / 27.0_f64 * t101907 - 176.0_f64 / 27.0_f64 * t95314 + 176.0_f64 / 27.0_f64 * t101929;
    t110027
}
