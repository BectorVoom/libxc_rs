//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1927;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1928;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta414(t1107: f64, t14758: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t14728: f64, t14809: f64, t14811: f64, t14814: f64, t14816: f64, t14818: f64, t11211: f64, t11213: f64, t11369: f64, t11372: f64, t14702: f64, t14705: f64, t14708: f64, t14711: f64, t14713: f64, t14759: f64, t14776: f64, t14779: f64, t14782: f64, t14784: f64, t14787: f64, t14790: f64, t14793: f64, t14796: f64, t14799: f64, t14802: f64, t14805: f64, t1147: f64, t1156: f64, t1164: f64, t3423: f64, t4869: f64, t11126: f64, t1703: f64, t1657: f64, t3263: f64, t3266: f64, t11292: f64, t1694: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14824, t14827) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1927(t1107, t14758, t11137, t11139, t11141, t11143, t14728, t14809, t14811, t14814, t14816, t14818);
        let t14829 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1928(t11211, t11213, t11369, t11372, t14702, t14705, t14708, t14711, t14713, t14759, t14776, t14779, t14782, t14784, t14787, t14790, t14793, t14796, t14799, t14802, t14805, t14827);
        let (t14831, t14833, t14835, t14837, t14838, t14840, t14841) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1929(t1147, t1156, t14829, t1164, t3423, t4869, t11126, t1703, t1657, t3263, t3266, t11292, t1694);
    (t14824, t14829, t14831, t14833, t14835, t14837, t14838, t14840, t14841)
}
