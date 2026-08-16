//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2330;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2331;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2332;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta533(t16696: f64, t5332: f64, t3720: f64, t12772: f64, t5406: f64, t3625: f64, t1248: f64, t5245: f64, t1250: f64, t1802: f64, t474: f64, t3089: f64, t3717: f64, t1261: f64, t12809: f64, t12832: f64, t17362: f64, t17369: f64, t17375: f64, t17377: f64, t3613: f64, t3647: f64, t3718: f64, t3723: f64, t5348: f64, t5354: f64, t5397: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17380, t17381, t17384, t17386, t17389, t17390, t17391, t17394) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2330(t16696, t5332, t3720, t12772, t5406, t3625, t1248, t5245, t1250, t1802, t474);
        let t17395 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2331(t17394, t3089);
        let t17396 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2332(t17395, t3717);
        let t17399 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2333(t1261, t12809, t12832, t17362, t17369, t17375, t17377, t17381, t17386, t17391, t17396, t3613, t3647, t3718, t3723, t5348, t5354, t5397);
    (t17380, t17381, t17384, t17386, t17389, t17390, t17391, t17394, t17395, t17396, t17399)
}
