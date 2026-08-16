//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1106/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1106(t5: f64, t1493: f64, t33275: f64, t8621: f64, t32798: f64, t32802: f64, t33283: f64, t34402: f64, t34410: f64, t34761: f64, t34765: f64, t8737: f64, t8882: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t34771 = t8621 * t33275 * t1493;
    let t34775 = piecewise3(t8, 0.0_f64, -5.0_f64 / 72.0_f64 * t34402 * t8882 + 5.0_f64 / 12.0_f64 * t32798 * t34761 + 5.0_f64 / 18.0_f64 * t32802 * t34765 - 5.0_f64 / 72.0_f64 * t34410 * t8882 - 5.0_f64 / 36.0_f64 * t8737 * t34771 + t33283);
    (t34771, t34775)
}
