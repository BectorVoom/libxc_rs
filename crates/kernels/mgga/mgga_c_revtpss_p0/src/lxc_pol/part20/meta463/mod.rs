//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1760;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1761;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1762;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta463<F: Float>(t72: F, t9940: F, t245: F, t543: F, t9400: F, t2713: F, t3964: F, t9714: F, t3951: F, t9732: F, t1353: F, t9994: F, t136: F, t4010: F, t220: F, t1399: F, t3945: F, t9816: F, t13804: F, t3889: F, t3934: F, t3936: F, t3937: F, t46416: F, t46655: F, t47216: F, t47221: F, t47223: F, t47227: F, t47229: F, t47231: F, t47235: F, t47239: F, t47245: F, t5673: F, t800: F, t9748: F, t9805: F, t9826: F, t9955: F, t9956: F, t13847: F, t4057: F, t9819: F, t9807: F, t9962: F, t9832: F, t2482: F, t27: F, t9991: F, t221: F, t4019: F, t9995: F, t9905: F, t9976: F, t9984: F, t3978: F, t9921: F, t3926: F, t9909: F, t3930: F, t9901: F, t2661: F, t5675: F, t9929: F, t9934: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47248, t47249, t47259, t47262, t47264) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1760::<F>(t72, t9940, t245, t543, t9400, t2713, t3964, t9714, t3951, t9732, t1353, t9994);
        let t47279 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1761::<F>(t136, t4010, t220, t1399, t3945, t9816, t13804, t3889, t3934, t3936, t3937, t46416, t46655, t47216, t47221, t47223, t47227, t47229, t47231, t47235, t47239, t47245, t47248, t47249, t47259, t47262, t47264, t5673, t800, t9748, t9805, t9826, t9955, t9956);
        let (t47282, t47284, t47286, t47296) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1762::<F>(t13847, t4057, t9816, t9819, t9807, t9962, t9832, t2482, t27, t9991, t221, t4019, t9995);
        let (t47298, t47302, t47304, t47306, t47318) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1763::<F>(t9905, t9976, t221, t9984, t3978, t9921, t3926, t9909, t3930, t9901, t2661, t5675, t9929, t9934);
    (t47279, t47282, t47284, t47286, t47296, t47298, t47302, t47304, t47306, t47318)
}
