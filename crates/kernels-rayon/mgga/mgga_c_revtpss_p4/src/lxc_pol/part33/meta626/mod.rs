//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2068;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta626(t99011: f64, t4349: f64, t93072: f64, t14673: f64, t92955: f64, t14688: f64, t4452: f64, t92951: f64, t14719: f64, t25227: f64, t2661: f64, t14723: f64, t25266: f64, t4426: f64, t1561: f64, t93048: f64, t14741: f64, t1945: f64, t807: f64, t10886: f64, t4416: f64, t7028: f64, t1549: f64, t92968: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99012, t99013, t99020, t99022, t99024, t99027, t99029) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2068(t99011, t4349, t93072, t14673, t92955, t14688, t4452, t92951, t14719, t25227, t2661, t14723);
        let (t99030, t99034, t99035, t99042, t99044, t99050) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2069(t99029, t25266, t4426, t1561, t93048, t14741, t1945, t807, t10886, t4416, t7028, t1549, t92968);
    (t99012, t99013, t99020, t99022, t99024, t99027, t99030, t99034, t99035, t99042, t99044, t99050)
}
