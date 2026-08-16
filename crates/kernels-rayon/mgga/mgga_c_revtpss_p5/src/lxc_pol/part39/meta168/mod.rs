//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta168 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk746;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk747;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk748;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta168(t3362: f64, t3698: f64, t2251: f64, t1012: f64, t1251: f64, t3172: f64, t1247: f64, t1032: f64, t1204: f64, t1246: f64, t1234: f64, t1260: f64, t1214: f64, t1263: f64, t1122: f64, t1042: f64, t1209: f64, t1284: f64, t3624: f64, t482: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3700, t3701, t3704, t3705, t3707, t3708) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk746(t3362, t3698, t2251, t1012, t1251, t3172, t1247, t1032, t1204, t1246);
        let t3711 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk747(t1234, t1260);
        let (t3713, t3714, t3717, t3718) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk748(t1214, t1263, t1122, t1042, t1209, t1284, t3624);
        let t3719 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk749(t482, t66);
    (t3700, t3701, t3704, t3705, t3707, t3708, t3711, t3713, t3714, t3717, t3718, t3719)
}
