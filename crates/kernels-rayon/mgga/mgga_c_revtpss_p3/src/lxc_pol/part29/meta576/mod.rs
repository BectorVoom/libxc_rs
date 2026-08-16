//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1924;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta576(t4452: f64, t92951: f64, t14719: f64, t25227: f64, t2661: f64, t14723: f64, t14774: f64, t7045: f64, t25266: f64, t4426: f64, t1561: f64, t93048: f64, t14741: f64, t1945: f64, t807: f64, t10886: f64, t4416: f64, t7028: f64, t27221: f64, t50789: f64, t50931: f64, t1549: f64, t92968: f64, t14697: f64, t25270: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99023, t99026, t99029, t99031, t99033, t99035) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1924(t4452, t92951, t14719, t25227, t2661, t14723, t14774, t7045, t25266, t4426, t1561, t93048);
        let (t99041, t99044, t99046, t99048, t99050, t99052) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1925(t14741, t1945, t807, t10886, t4416, t7028, t27221, t50789, t50931, t1549, t92968, t14697, t25270);
    (t99023, t99026, t99029, t99031, t99033, t99035, t99041, t99044, t99046, t99048, t99050, t99052)
}
