//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1648;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1649;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta336(t240: f64, t3252: f64, t11145: f64, t141: f64, t11169: f64, t930: f64, t11158: f64, t11162: f64, t11167: f64, t11316: f64, t11319: f64, t11322: f64, t11326: f64, t11329: f64, t11332: f64, t11334: f64, t11338: f64, t11339: f64, t276: f64, t285: f64, t2881: f64, t918: f64, t273: f64, t2439: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11341, t11342, t11343, t11345, t11346, t11349) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1648(t240, t3252, t11145, t141, t11169, t930, t11158, t11162, t11167, t11316, t11319, t11322, t11326, t11329, t11332, t11334, t11338, t11339);
        let (t11354, t11355, t11356, t11358, t11359, t11366) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1649(t276, t285, t2881, t918, t273, t2439, t931);
    (t11341, t11342, t11343, t11345, t11346, t11349, t11354, t11355, t11356, t11358, t11359, t11366)
}
