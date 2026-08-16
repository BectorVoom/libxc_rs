//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2187/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2187(t25978: f64, t6856: f64, t102569: f64, t108615: f64, t108617: f64, t108619: f64, t108623: f64, t108625: f64, t108627: f64, t94554: f64, t94565: f64, t94569: f64, t94571: f64, t98282: f64) -> f64 {
    let t108629 = t25978 * t6856;
    let t108631 = -0.15244095330869239812e-3_f64 * t94554 + t108615 / 16.0_f64 - t108617 / 4.0_f64 + t108619 / 8.0_f64 + t98282 - 0.90357964994909313586e-5_f64 * t94565 - t94569 - t94571 - t102569 + 0.14291339372689912324e-4_f64 * t108623 + 0.50820002809285328226e-3_f64 * t108625 - 0.40015750243531754508e-1_f64 * t108627 + 0.80031500487063509015e-2_f64 * t108629;
    t108631
}
