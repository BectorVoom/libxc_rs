//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta644 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2122;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2123;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2124;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2125;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2126;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta644(t2631: f64, t47285: f64, t6605: f64, t9972: f64, t12971: f64, t1894: f64, t236: f64, t6591: f64, t23046: f64, t4184: f64, t812: f64, t836: f64, t13080: f64, t23146: f64, t242: f64, t81816: f64, t13265: f64, t13333: f64, t25084: f64, t13076: f64, t13084: f64, t87329: f64, t87331: f64, t87333: f64, t87336: f64, t87339: f64, t87342: f64, t87343: f64, t87345: f64, t87348: f64, t87351: f64, t25083: f64, t2617: f64, t13244: f64, t25064: f64, t81788: f64, t13193: f64, t6621: f64, t13198: f64, t23097: f64, t232: f64, t46565: f64, t815: f64, t46644: f64, t25135: f64, t838: f64, t2693: f64, t7503: f64, t25132: f64, t81882: f64, t6604: f64, t81968: f64, t13184: f64, t841: f64, t23083: f64, t25123: f64, t81912: f64, t81887: f64, t81889: f64, t81899: f64, t81903: f64, t81909: f64, t13191: f64, t25119: f64, t1878: f64, t81982: f64, t221: f64, t25120: f64, t81962: f64, t13196: f64, t13204: f64, t6581: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t87355, t87359, t87363) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2122(t2631, t47285, t6605, t9972, t12971, t1894, t236, t6591, t23046, t4184, t812, t836);
        let t87377 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2123(t87363, t13080, t23146, t242, t812, t81816, t13265, t13333, t25084, t13076, t13084, t87329, t87331, t87333, t87336, t87339, t87342, t87343, t87345, t87348, t87351, t87355, t87359);
        let (t87379, t87381, t87387, t87389, t87391, t87395) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2124(t25083, t2617, t4184, t13244, t25084, t25064, t81788, t13193, t6621, t13198, t23097, t232, t46565, t815);
        let (t87399, t87402, t87403, t87405, t87407) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2125(t23097, t232, t46644, t815, t25135, t838, t2693, t7503, t25132, t81882, t6604, t81968);
        let t87415 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2126(t13184, t841, t87407, t23083, t25123, t81912, t81887, t81889, t81899, t81903, t81909, t87379, t87381, t87387, t87389, t87391, t87395, t87399, t87402, t87403, t87405);
        let (t87418, t87422, t87426, t87428, t87430) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2127(t13191, t25119, t841, t1878, t81982, t13184, t221, t25120, t6604, t81962, t13196, t13204, t6581);
    (t87377, t87415, t87418, t87422, t87426, t87428, t87430)
}
