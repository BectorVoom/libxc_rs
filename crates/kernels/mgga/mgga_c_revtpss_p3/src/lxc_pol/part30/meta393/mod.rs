//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1477;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1478;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta393<F: Float>(t13610: F, t13638: F, t13663: F, t14308: F, t1532: F, t2609: F, t10437: F, t2398: F, t4308: F, t4305: F, t262: F, t4343: F, t177: F, t4392: F, t762: F, t10605: F, t162: F, t4403: F, t2626: F, t4398: F, t10439: F, t2251: F, t4402: F, t2516: F, t2496: F, t10443: F, t10552: F, t10554: F, t4541: F, t775: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14310, t14312, t14313, t14315, t14317, t14318) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1477::<F>(t13610, t13638, t13663, t14308, t1532, t2609, t10437, t2398, t4308, t4305, t262, t4343);
        let (t14324, t14327, t14329, t14333, t14334) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1478::<F>(t177, t4392, t762, t10605, t162, t4403, t2626, t4398, t10439, t2251, t4402, t2516);
        let (t14335, t14337, t14338) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1479::<F>(t14334, t2496, t4398, t10443, t10552, t10554, t14312, t14313, t14315, t14317, t14318, t14324, t14327, t14329, t14333, t4541, t775, t9278, t9308, t9316, t9329, t9333);
    (t14310, t14312, t14313, t14315, t14317, t14324, t14327, t14329, t14333, t14335, t14337, t14338)
}
