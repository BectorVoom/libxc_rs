//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1475;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1476;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1477;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta418(t31439: f64, t8315: f64, t1509: f64, t661: f64, t31149: f64, t2: f64, t31035: f64, t31134: f64, t31135: f64, t31137: f64, t31287: f64, t31415: f64, t31417: f64, t31421: f64, t31424: f64, t31427: f64, t31430: f64, t31434: f64, t31437: f64, t8258: f64, t8267: f64, t114: f64, t508: f64, t1911: f64, t8320: f64, t569: f64, t1312: f64, t13426: f64, t18227: f64, t2201: f64, t2322: f64, t27123: f64, t31401: f64, t31403: f64, t31407: f64, t4248: f64, t4254: f64, t5523: f64, t651: f64, t8307: f64, t8325: f64, t8327: f64, t8407: f64, t8413: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31440, t31443, t31444, t31447, t31450) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1475(t31439, t8315, t1509, t661, t31149, t2, t31035, t31134, t31135, t31137, t31287, t31415, t31417, t31421, t31424, t31427, t31430, t31434, t31437, t8258, t8267);
        let t31451 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1476(t114, t31450);
        let (t31452, t31456, t31459, t31461) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1477(t31451, t508, t1911, t8320, t569, t1312, t13426, t18227, t2201, t2322, t27123, t31401, t31403, t31407, t4248, t4254, t5523, t651, t8307, t8325, t8327, t8407, t8413);
    (t31440, t31443, t31444, t31447, t31451, t31452, t31456, t31459, t31461)
}
