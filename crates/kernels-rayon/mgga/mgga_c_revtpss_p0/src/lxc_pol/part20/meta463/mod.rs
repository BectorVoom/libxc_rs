//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1760;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1761;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1762;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta463(t72: f64, t9940: f64, t245: f64, t543: f64, t9400: f64, t2713: f64, t3964: f64, t9714: f64, t3951: f64, t9732: f64, t1353: f64, t9994: f64, t136: f64, t4010: f64, t220: f64, t1399: f64, t3945: f64, t9816: f64, t13804: f64, t3889: f64, t3934: f64, t3936: f64, t3937: f64, t46416: f64, t46655: f64, t47216: f64, t47221: f64, t47223: f64, t47227: f64, t47229: f64, t47231: f64, t47235: f64, t47239: f64, t47245: f64, t5673: f64, t800: f64, t9748: f64, t9805: f64, t9826: f64, t9955: f64, t9956: f64, t13847: f64, t4057: f64, t9819: f64, t9807: f64, t9962: f64, t9832: f64, t2482: f64, t27: f64, t9991: f64, t221: f64, t4019: f64, t9995: f64, t9905: f64, t9976: f64, t9984: f64, t3978: f64, t9921: f64, t3926: f64, t9909: f64, t3930: f64, t9901: f64, t2661: f64, t5675: f64, t9929: f64, t9934: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47248, t47249, t47259, t47262, t47264) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1760(t72, t9940, t245, t543, t9400, t2713, t3964, t9714, t3951, t9732, t1353, t9994);
        let t47279 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1761(t136, t4010, t220, t1399, t3945, t9816, t13804, t3889, t3934, t3936, t3937, t46416, t46655, t47216, t47221, t47223, t47227, t47229, t47231, t47235, t47239, t47245, t47248, t47249, t47259, t47262, t47264, t5673, t800, t9748, t9805, t9826, t9955, t9956);
        let (t47282, t47284, t47286, t47296) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1762(t13847, t4057, t9816, t9819, t9807, t9962, t9832, t2482, t27, t9991, t221, t4019, t9995);
        let (t47298, t47302, t47304, t47306, t47318) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1763(t9905, t9976, t221, t9984, t3978, t9921, t3926, t9909, t3930, t9901, t2661, t5675, t9929, t9934);
    (t47279, t47282, t47284, t47286, t47296, t47298, t47302, t47304, t47306, t47318)
}
