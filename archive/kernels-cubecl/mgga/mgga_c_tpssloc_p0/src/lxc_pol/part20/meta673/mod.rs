//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta673 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2538;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2539;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2540;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta673<F: Float>(t1682: F, t3357: F, t11310: F, t1694: F, t3401: F, t11420: F, t1098: F, t14956: F, t1119: F, t14845: F, t3308: F, t3312: F, t4737: F, t3316: F, t11300: F, t11361: F, t11430: F, t11437: F, t11441: F, t1155: F, t15126: F, t15219: F, t15222: F, t43984: F, t44188: F, t4862: F, t51133: F, t51245: F, t51248: F, t51251: F, t11419: F, t1675: F, t11424: F, t15054: F, t15057: F, t44162: F, t11185: F, t15064: F, t15068: F, t43964: F, t3264: F, t3307: F, t4782: F, t11190: F, t15060: F, t3265: F, t11129: F, t11306: F, t11307: F, t11350: F, t11415: F, t11421: F, t15146: F, t15210: F, t15226: F, t15229: F, t1683: F, t3333: F, t44220: F, t4820: F, t4823: F, t4861: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51382, t51385, t51389, t51392, t51399, t51401, t51402) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2538::<F>(t1682, t3357, t11310, t1694, t3401, t11420, t1098, t14956, t1119, t14845, t3308, t3312, t4737);
        let (t51404, t51411) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2539::<F>(t3316, t51402, t11300, t11361, t11430, t11437, t11441, t1155, t15126, t15219, t15222, t43984, t44188, t4862, t51133, t51245, t51248, t51251, t51382, t51385, t51389, t51392, t51399, t51401);
        let (t51427, t51437, t51439, t51441, t51443, t51446) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2540::<F>(t11419, t1675, t11424, t15054, t15057, t44162, t11185, t15064, t15068, t43964, t3264, t3307, t4782);
        let (t51449, t51450) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2541::<F>(t11190, t15060, t3265, t11129, t11306, t11307, t11310, t11350, t11361, t11415, t11420, t11421, t15146, t15210, t15226, t15229, t1683, t3333, t3357, t44220, t4820, t4823, t4861, t51427, t51437, t51439, t51441, t51443, t51446);
    (t51399, t51401, t51404, t51411, t51437, t51439, t51441, t51443, t51446, t51449, t51450)
}
