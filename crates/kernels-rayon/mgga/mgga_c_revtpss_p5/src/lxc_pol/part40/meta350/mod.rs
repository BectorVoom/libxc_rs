//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1201;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1202;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1203;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta350(t13750: f64, t14088: f64, t14279: f64, t14302: f64, t1343: f64, t13664: f64, t13667: f64, t13669: f64, t13671: f64, t13673: f64, t13674: f64, t13682: f64, t13683: f64, t13716: f64, t13885: f64, t13886: f64, t13888: f64, t1450: f64, t198: f64, t3889: f64, t4135: f64, t4139: f64, t4144: f64, t532: f64, t5532: f64, t5541: f64, t5542: f64, t9524: f64, t9542: f64, t9854: f64, t9865: f64, t9868: f64, t13610: f64, t13638: f64, t13663: f64, t1532: f64, t2609: f64, t10437: f64, t2398: f64, t4308: f64, t4305: f64, t262: f64, t4343: f64, t177: f64, t4392: f64, t762: f64, t10605: f64, t162: f64, t4403: f64, t2626: f64, t4398: f64, t10439: f64, t2251: f64, t4402: f64, t2516: f64, t2496: f64, t10443: f64, t10552: f64, t10554: f64, t4541: f64, t775: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t14308 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1201(t13750, t14088, t14279, t14302, t1343, t13664, t13667, t13669, t13671, t13673, t13674, t13682, t13683, t13716, t13885, t13886, t13888, t1450, t198, t3889, t4135, t4139, t4144, t532, t5532, t5541, t5542, t9524, t9542, t9854, t9865, t9868);
        let (t14310, t14312, t14313, t14315, t14317, t14318) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1202(t13610, t13638, t13663, t14308, t1532, t2609, t10437, t2398, t4308, t4305, t262, t4343);
        let (t14324, t14327, t14329, t14333, t14334) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1203(t177, t4392, t762, t10605, t162, t4403, t2626, t4398, t10439, t2251, t4402, t2516);
        let (t14335, t14337, t14338) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1204(t14334, t2496, t4398, t10443, t10552, t10554, t14312, t14313, t14315, t14317, t14318, t14324, t14327, t14329, t14333, t4541, t775, t9278, t9308, t9316, t9329, t9333);
    (t14310, t14312, t14313, t14315, t14317, t14324, t14327, t14329, t14333, t14335, t14337, t14338)
}
