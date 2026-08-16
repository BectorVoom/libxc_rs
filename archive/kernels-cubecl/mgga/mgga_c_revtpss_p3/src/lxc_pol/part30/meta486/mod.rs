//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1827;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1828;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1829;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta486<F: Float>(t25949: F, t7063: F, t7286: F, t2470: F, t7285: F, t7289: F, t2030: F, t25882: F, t25885: F, t25889: F, t25893: F, t25896: F, t25902: F, t25905: F, t25909: F, t25914: F, t25919: F, t25921: F, t25926: F, t25930: F, t25934: F, t25941: F, t25948: F, t4132: F, t7279: F, t7292: F, t7295: F, t7298: F, t7308: F, t1398: F, t543: F, t7274: F, t7301: F, t2022: F, t4056: F, t3974: F, t7259: F, t2482: F, t27: F, t7269: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25950, t25951, t25953) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1827::<F>(t25949, t7063, t7286, t2470, t7285);
        let (t25955, t25956) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1828::<F>(t25953, t7289, t2030, t25882, t25885, t25889, t25893, t25896, t25902, t25905, t25909, t25914, t25919, t25921, t25926, t25930, t25934, t25941, t25948, t25951, t4132, t7279, t7292, t7295, t7298, t7308);
        let (t25961, t25966, t25970, t25972) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1829::<F>(t1398, t543, t7274, t7301, t2022, t4056, t3974, t7259, t2482, t27, t7269);
    (t25950, t25951, t25953, t25955, t25956, t25961, t25966, t25970, t25972)
}
