//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1871;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1872;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta494(t25949: f64, t7063: f64, t7286: f64, t2470: f64, t7285: f64, t7289: f64, t2030: f64, t25882: f64, t25885: f64, t25889: f64, t25893: f64, t25896: f64, t25902: f64, t25905: f64, t25909: f64, t25914: f64, t25919: f64, t25921: f64, t25926: f64, t25930: f64, t25934: f64, t25941: f64, t25948: f64, t4132: f64, t7279: f64, t7292: f64, t7295: f64, t7298: f64, t7308: f64, t1398: f64, t543: f64, t7274: f64, t7301: f64, t2022: f64, t4056: f64, t3974: f64, t7259: f64, t2482: f64, t27: f64, t7269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25950, t25951, t25953) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1871(t25949, t7063, t7286, t2470, t7285);
        let (t25955, t25956) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1872(t25953, t7289, t2030, t25882, t25885, t25889, t25893, t25896, t25902, t25905, t25909, t25914, t25919, t25921, t25926, t25930, t25934, t25941, t25948, t25951, t4132, t7279, t7292, t7295, t7298, t7308);
        let (t25961, t25966, t25970, t25972) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1873(t1398, t543, t7274, t7301, t2022, t4056, t3974, t7259, t2482, t27, t7269);
    (t25950, t25951, t25953, t25955, t25956, t25961, t25966, t25970, t25972)
}
