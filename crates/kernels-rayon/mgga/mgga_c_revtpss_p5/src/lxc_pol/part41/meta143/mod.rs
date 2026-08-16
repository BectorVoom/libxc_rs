//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta143 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk660;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk661;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta143(t1412: f64, t72: f64, t245: f64, t1353: f64, t543: f64, t159: f64, t550: f64, t216: f64, t1376: f64, t2689: f64, t1413: f64, t547: f64, t807: f64, t2700: f64, t535: f64, t1369: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3935, t3936) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk660(t1412, t72, t245);
        let t3938 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk661(t1353, t543);
        let (t3943, t3944, t3950, t3951, t3952, t3953, t3956, t3957) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk662(t159, t550, t216, t1376, t2689, t1353, t1413, t547, t807, t2700, t535, t1369, t794);
    (t3935, t3936, t3938, t3943, t3944, t3950, t3951, t3952, t3953, t3956, t3957)
}
