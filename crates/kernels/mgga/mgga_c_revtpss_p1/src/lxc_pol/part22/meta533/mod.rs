//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2330;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2331;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2332;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta533<F: Float>(t16696: F, t5332: F, t3720: F, t12772: F, t5406: F, t3625: F, t1248: F, t5245: F, t1250: F, t1802: F, t474: F, t3089: F, t3717: F, t1261: F, t12809: F, t12832: F, t17362: F, t17369: F, t17375: F, t17377: F, t3613: F, t3647: F, t3718: F, t3723: F, t5348: F, t5354: F, t5397: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17380, t17381, t17384, t17386, t17389, t17390, t17391, t17394) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2330::<F>(t16696, t5332, t3720, t12772, t5406, t3625, t1248, t5245, t1250, t1802, t474);
        let t17395 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2331::<F>(t17394, t3089);
        let t17396 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2332::<F>(t17395, t3717);
        let t17399 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2333::<F>(t1261, t12809, t12832, t17362, t17369, t17375, t17377, t17381, t17386, t17391, t17396, t3613, t3647, t3718, t3723, t5348, t5354, t5397);
    (t17380, t17381, t17384, t17386, t17389, t17390, t17391, t17394, t17395, t17396, t17399)
}
