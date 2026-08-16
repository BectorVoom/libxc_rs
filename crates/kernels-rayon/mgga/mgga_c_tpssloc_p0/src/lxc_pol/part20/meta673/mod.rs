//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta673 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2538;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2539;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2540;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta673(t1682: f64, t3357: f64, t11310: f64, t1694: f64, t3401: f64, t11420: f64, t1098: f64, t14956: f64, t1119: f64, t14845: f64, t3308: f64, t3312: f64, t4737: f64, t3316: f64, t11300: f64, t11361: f64, t11430: f64, t11437: f64, t11441: f64, t1155: f64, t15126: f64, t15219: f64, t15222: f64, t43984: f64, t44188: f64, t4862: f64, t51133: f64, t51245: f64, t51248: f64, t51251: f64, t11419: f64, t1675: f64, t11424: f64, t15054: f64, t15057: f64, t44162: f64, t11185: f64, t15064: f64, t15068: f64, t43964: f64, t3264: f64, t3307: f64, t4782: f64, t11190: f64, t15060: f64, t3265: f64, t11129: f64, t11306: f64, t11307: f64, t11350: f64, t11415: f64, t11421: f64, t15146: f64, t15210: f64, t15226: f64, t15229: f64, t1683: f64, t3333: f64, t44220: f64, t4820: f64, t4823: f64, t4861: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51382, t51385, t51389, t51392, t51399, t51401, t51402) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2538(t1682, t3357, t11310, t1694, t3401, t11420, t1098, t14956, t1119, t14845, t3308, t3312, t4737);
        let (t51404, t51411) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2539(t3316, t51402, t11300, t11361, t11430, t11437, t11441, t1155, t15126, t15219, t15222, t43984, t44188, t4862, t51133, t51245, t51248, t51251, t51382, t51385, t51389, t51392, t51399, t51401);
        let (t51427, t51437, t51439, t51441, t51443, t51446) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2540(t11419, t1675, t11424, t15054, t15057, t44162, t11185, t15064, t15068, t43964, t3264, t3307, t4782);
        let (t51449, t51450) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2541(t11190, t15060, t3265, t11129, t11306, t11307, t11310, t11350, t11361, t11415, t11420, t11421, t15146, t15210, t15226, t15229, t1683, t3333, t3357, t44220, t4820, t4823, t4861, t51427, t51437, t51439, t51441, t51443, t51446);
    (t51399, t51401, t51404, t51411, t51437, t51439, t51441, t51443, t51446, t51449, t51450)
}
