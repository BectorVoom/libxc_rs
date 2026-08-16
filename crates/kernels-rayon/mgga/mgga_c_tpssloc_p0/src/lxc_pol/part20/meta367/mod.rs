//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta367 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1706;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1707;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1708;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1709;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1710;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta367(t584: f64, t95: f64, t16: f64, t4053: f64, t1449: f64, t2350: f64, t9398: f64, t100: f64, t2349: f64, t2219: f64, t662: f64, t2354: f64, t4059: f64, t103: f64, t4063: f64, t12771: f64, t12774: f64, t12775: f64, t12778: f64, t1445: f64, t1447: f64, t2336: f64, t2351: f64, t2355: f64, t4050: f64, t4054: f64, t657: f64, t92: f64, t656: f64, t12747: f64, t12750: f64, t12752: f64, t12754: f64, t12758: f64, t12761: f64, t64: f64, t9358: f64, t9359: f64, t9361: f64, t9363: f64, t109: f64, t1268: f64, t12724: f64, t12725: f64, t12728: f64, t12734: f64, t12739: f64, t1458: f64, t2314: f64, t2363: f64, t4028: f64, t4072: f64, t5113: f64, t671: f64, t9348: f64, t89: f64, t12545: f64, t12550: f64, t12557: f64, t1442: f64, t1459: f64, t1849: f64, t2323: f64, t2364: f64, t3652: f64, t3660: f64, t4034: f64, t4037: f64, t4073: f64, t574: f64, t652: f64, t672: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12781, t12784, t12792, t12795, t12796, t12799) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1706(t584, t95, t16, t4053, t1449, t2350, t9398, t100, t2349, t2219, t662, t2354, t4059);
        let t12808 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1707(t103, t584, t16, t4063, t100, t12771, t12774, t12775, t12778, t12781, t12784, t12792, t12795, t12796, t12799, t1445, t1447, t2336, t2351, t2355, t4050, t4054, t657, t92);
        let (t12809, t12812) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1708(t12808, t656, t12747, t12750, t12752, t12754, t12758, t12761, t64, t9358, t9359, t9361, t9363);
        let t12813 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1709(t109, t12812);
        let t12816 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1710(t1268, t12724, t12725, t12728, t12734, t12739, t12813, t1458, t2314, t2363, t4028, t4072, t5113, t671, t9348);
        let (t12823, t12832) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1711(t2363, t89, t12545, t12550, t12557, t12725, t12734, t12816, t1442, t1459, t1849, t2314, t2323, t2364, t3652, t3660, t4028, t4034, t4037, t4073, t574, t652, t672, t9348);
    (t12781, t12784, t12795, t12808, t12809, t12813, t12816, t12823, t12832)
}
