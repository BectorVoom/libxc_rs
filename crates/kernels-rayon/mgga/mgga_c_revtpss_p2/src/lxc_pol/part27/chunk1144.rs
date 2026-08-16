//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1144/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1144(t25970: f64, t25974: f64, t25976: f64, t25980: f64, t25984: f64, t25989: f64, t25990: f64, t25992: f64, t25994: f64, t25998: f64, t26033: f64) -> f64 {
    let t26034 = -t25970 - t25974 + t25976 + t25980 + 0.85748036236139473944e-3_f64 * t25984 + t25989 - 0.17149607247227894789e-2_f64 * t25990 + 0.85748036236139473945e-2_f64 * t25992 - 0.42874018118069736972e-3_f64 * t25994 - 0.50820002809285328226e-4_f64 * t25998 + t26033;
    t26034
}
