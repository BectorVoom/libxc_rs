//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2057;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2058;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2059;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta474(t119: f64, t16018: f64, t210: f64, t12308: f64, t12310: f64, t12317: f64, t12323: f64, t12325: f64, t12330: f64, t12336: f64, t1315: f64, t1363: f64, t1369: f64, t16321: f64, t16325: f64, t16331: f64, t16333: f64, t16338: f64, t16341: f64, t16346: f64, t16347: f64, t16350: f64, t16354: f64, t1831: f64, t3783: f64, t3876: f64, t5240: f64, t5314: f64, t559: f64, t120: f64, t5187: f64, t1352: f64, t3805: f64, t3851: f64, t5301: f64, t1810: f64, t3734: f64, t3856: f64, t3793: f64, t5248: f64, t5249: f64, t3802: f64, t5234: f64, t3788: f64, t836: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16356, t16361) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2057(t119, t16018, t210, t12308, t12310, t12317, t12323, t12325, t12330, t12336, t1315, t1363, t1369, t16321, t16325, t16331, t16333, t16338, t16341, t16346, t16347, t16350, t16354, t1831, t3783, t3876, t5240, t5314, t559);
        let (t16366, t16370, t16379, t16383, t16387) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2058(t120, t5187, t1352, t3805, t3851, t5301, t1810, t210, t3734, t3856, t3793, t5248, t5249);
        let (t16391, t16394) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2059(t3793, t3805, t5301, t3802, t5234);
        let (t16397, t16398) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2060(t3788, t836, t1336);
    (t16356, t16361, t16366, t16370, t16379, t16383, t16387, t16391, t16394, t16397, t16398)
}
