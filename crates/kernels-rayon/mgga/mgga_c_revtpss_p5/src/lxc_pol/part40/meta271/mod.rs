//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1008;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1009;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta271(t3911: f64, t3920: f64, t3957: f64, t3961: f64, t3829: f64, t4011: f64, t547: f64, t807: f64, t2237: f64, t240: f64, t550: f64, t816: f64, t1379: f64, t2689: f64, t3952: f64, t1413: f64, t3889: f64, t9646: f64, t2236: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9695, t9697, t9705, t9707, t9709) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1008(t3911, t3920, t3957, t3961, t3829, t4011, t547, t807, t2237, t240, t550, t816);
        let (t9711, t9712, t9716, t9718, t9720, t9721) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1009(t1379, t9709, t2689, t3952, t1413, t3889, t547, t807, t9646, t2236, t66, t240);
    (t9695, t9697, t9705, t9707, t9711, t9712, t9716, t9718, t9720, t9721)
}
