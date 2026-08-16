//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1201;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1202;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1203;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta350<F: Float>(t13750: F, t14088: F, t14279: F, t14302: F, t1343: F, t13664: F, t13667: F, t13669: F, t13671: F, t13673: F, t13674: F, t13682: F, t13683: F, t13716: F, t13885: F, t13886: F, t13888: F, t1450: F, t198: F, t3889: F, t4135: F, t4139: F, t4144: F, t532: F, t5532: F, t5541: F, t5542: F, t9524: F, t9542: F, t9854: F, t9865: F, t9868: F, t13610: F, t13638: F, t13663: F, t1532: F, t2609: F, t10437: F, t2398: F, t4308: F, t4305: F, t262: F, t4343: F, t177: F, t4392: F, t762: F, t10605: F, t162: F, t4403: F, t2626: F, t4398: F, t10439: F, t2251: F, t4402: F, t2516: F, t2496: F, t10443: F, t10552: F, t10554: F, t4541: F, t775: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t14308 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1201::<F>(t13750, t14088, t14279, t14302, t1343, t13664, t13667, t13669, t13671, t13673, t13674, t13682, t13683, t13716, t13885, t13886, t13888, t1450, t198, t3889, t4135, t4139, t4144, t532, t5532, t5541, t5542, t9524, t9542, t9854, t9865, t9868);
        let (t14310, t14312, t14313, t14315, t14317, t14318) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1202::<F>(t13610, t13638, t13663, t14308, t1532, t2609, t10437, t2398, t4308, t4305, t262, t4343);
        let (t14324, t14327, t14329, t14333, t14334) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1203::<F>(t177, t4392, t762, t10605, t162, t4403, t2626, t4398, t10439, t2251, t4402, t2516);
        let (t14335, t14337, t14338) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1204::<F>(t14334, t2496, t4398, t10443, t10552, t10554, t14312, t14313, t14315, t14317, t14318, t14324, t14327, t14329, t14333, t4541, t775, t9278, t9308, t9316, t9329, t9333);
    (t14310, t14312, t14313, t14315, t14317, t14324, t14327, t14329, t14333, t14335, t14337, t14338)
}
