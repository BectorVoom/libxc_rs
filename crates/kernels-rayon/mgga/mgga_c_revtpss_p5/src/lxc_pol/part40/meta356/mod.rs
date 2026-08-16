//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1225;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1226;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1227;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta356(t10577: f64, t10582: f64, t10584: f64, t10586: f64, t14385: f64, t14388: f64, t14392: f64, t14396: f64, t14428: f64, t14433: f64, t14434: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64, t1531: f64, t37: f64, t2612: f64, t4392: f64, t72: f64, t757: f64, t14425: f64, t150: f64, t190: f64, t10608: f64, t2258: f64, t4402: f64, t4401: f64, t2414: f64, t4311: f64, t10428: f64, t1522: f64, t10613: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t14442: f64, t14443: f64, t14444: f64, t9542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t14612 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1225(t10577, t10582, t10584, t10586, t14385, t14388, t14392, t14396, t14428, t14433, t14434, t9514, t9517, t9521, t9524);
        let (t14615, t14618, t14620, t14621, t14622) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1226(t1531, t37, t2612, t4392, t72, t757, t14425, t150, t190, t10608, t2258, t4402);
        let (t14624, t14626, t14628, t14629, t14630) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1227(t14622, t4401, t2414, t4311, t10428, t1522, t10613, t10592, t10596, t10604, t10611, t14442, t14443, t14444, t14615, t14618, t14620, t14621, t9542);
    (t14612, t14615, t14618, t14620, t14621, t14624, t14626, t14628, t14629, t14630)
}
