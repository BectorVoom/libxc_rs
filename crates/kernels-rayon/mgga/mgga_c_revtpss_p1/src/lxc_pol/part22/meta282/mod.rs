//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1692;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1693;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta282(t3850: f64, t72: f64, t757: f64, t2619: f64, t3825: f64, t1333: f64, t3857: f64, t1331: f64, t3863: f64, t2626: f64, t676: f64, t3869: f64, t2434: f64, t762: f64, t3860: f64, t1320: f64, t3855: f64, t186: f64, t685: f64, t793: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9563, t9564, t9566, t9569, t9570, t9572) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1692(t3850, t72, t757, t2619, t3825, t1333, t3857, t1331, t3863, t2626, t676);
        let (t9574, t9575) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1693(t3869, t9572, t2434, t762);
        let (t9577, t9578, t9580, t9586) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1694(t3869, t9575, t1331, t3860, t1320, t3855, t186, t685, t793);
    (t9563, t9564, t9566, t9569, t9570, t9572, t9574, t9575, t9577, t9578, t9580, t9586)
}
