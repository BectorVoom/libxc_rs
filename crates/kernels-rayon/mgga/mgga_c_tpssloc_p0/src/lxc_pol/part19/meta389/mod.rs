//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1462;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1463;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1464;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta389(t3242: f64, t415: f64, t61: f64, t42341: f64, t44696: f64, t42344: f64, t483: f64, t1210: f64, t1174: f64, t3561: f64, t698: f64, t11738: f64, t11739: f64, t248: f64, t3570: f64, t10471: f64, t44690: f64, t11727: f64, t44722: f64, t478: f64, t11719: f64, t11722: f64, t3507: f64, t486: f64, t11655: f64, t11731: f64, t11825: f64, t1214: f64, t1227: f64, t15615: f64, t15654: f64, t3490: f64, t3494: f64, t3555: f64, t3587: f64, t39097: f64, t39103: f64, t42468: f64, t43764: f64, t44699: f64, t44725: f64, t44803: f64, t44805: f64, t44811: f64, t44817: f64, t4582: f64, t475: f64, t974: f64, t11638: f64, t11818: f64, t1213: f64, t3506: f64, t3509: f64, t3515: f64, t3516: f64, t11718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44828, t44833, t44834, t44836, t44847, t44851) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1462(t3242, t415, t61, t42341, t44696, t42344, t483, t1210, t1174, t3561, t698, t11738, t11739, t248, t3570);
        let (t44857, t44858, t44863, t44871, t44873) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1463(t10471, t44690, t11727, t44722, t44833, t44834, t478, t11719, t11722, t248, t3570, t3507, t486);
        let t44878 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1464(t11655, t11731, t11738, t1174, t11825, t1214, t1227, t15615, t15654, t248, t3490, t3494, t3555, t3587, t39097, t39103, t42468, t43764, t44699, t44725, t44803, t44805, t44811, t44817, t44828, t44836, t44847, t44851, t44858, t44863, t44871, t44873, t4582, t475, t974);
        let (t44879, t44886, t44890, t44894, t44896) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1465(t11638, t486, t11818, t1213, t248, t3494, t3506, t3509, t3515, t3516, t11718, t44857);
    (t44833, t44834, t44857, t44873, t44878, t44879, t44886, t44890, t44894, t44896)
}
