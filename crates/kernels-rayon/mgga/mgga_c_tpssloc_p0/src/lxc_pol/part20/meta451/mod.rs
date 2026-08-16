//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1901;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1902;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1903;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta451(t15067: f64, t3265: f64, t11275: f64, t14704: f64, t14710: f64, t14720: f64, t11215: f64, t11217: f64, t14722: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64, t14766: f64, t14781: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t14728: f64, t14809: f64, t14811: f64, t14814: f64, t14816: f64, t14818: f64, t14824: f64, t11211: f64, t11213: f64, t11314: f64, t11317: f64, t14702: f64, t14708: f64, t14713: f64, t14759: f64, t14779: f64, t14784: f64, t14787: f64, t14790: f64, t14793: f64, t14796: f64, t14799: f64, t14802: f64, t14805: f64, t1137: f64, t1147: f64, t4832: f64, t1687: f64, t3400: f64, t1156: f64, t14829: f64, t3375: f64, t1129: f64, t11356: f64, t1148: f64, t1157: f64, t14840: f64, t14847: f64, t14849: f64, t14852: f64, t1695: f64, t3371: f64, t3378: f64, t3396: f64, t3404: f64, t4835: f64, t4858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15068, t15070, t15072, t15074, t15091) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1901(t15067, t3265, t11275, t14704, t14710, t14720, t11215, t11217, t14722, t14733, t14738, t14742, t14746, t14751, t14755, t14766);
        let (t15094, t15115) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1902(t14781, t11137, t11139, t11141, t11143, t14728, t14809, t14811, t14814, t14816, t14818, t14824);
        let t15117 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1903(t11211, t11213, t11314, t11317, t14702, t14708, t14713, t14759, t14779, t14784, t14787, t14790, t14793, t14796, t14799, t14802, t14805, t15072, t15074, t15091, t15094, t15115);
        let (t15118, t15121, t15126, t15133, t15136, t15139) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1904(t1137, t15117, t1147, t4832, t1687, t3400, t1156, t14829, t3375, t1129, t11356, t1148, t1157, t14840, t14847, t14849, t14852, t1695, t3371, t3378, t3396, t3404, t4835, t4858);
    (t15068, t15070, t15117, t15118, t15121, t15126, t15133, t15136, t15139)
}
