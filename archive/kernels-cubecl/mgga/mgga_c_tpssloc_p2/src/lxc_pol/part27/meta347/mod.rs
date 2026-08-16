//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1440;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1441;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1442;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1443;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1444;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1445;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1446;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1447;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1448;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta347<F: Float>(t12757: F, t666: F, t2358: F, t4043: F, t1444: F, t2342: F, t9384: F, t2341: F, t92: F, t2219: F, t659: F, t2248: F, t4049: F, t584: F, t95: F, t16: F, t4053: F, t1449: F, t2350: F, t9398: F, t100: F, t2349: F, t662: F, t2354: F, t4059: F, t103: F, t4063: F, t1445: F, t1447: F, t2336: F, t2351: F, t2355: F, t4050: F, t4054: F, t657: F, t656: F, t12747: F, t12750: F, t12752: F, t12754: F, t64: F, t9358: F, t9359: F, t9361: F, t9363: F, t109: F, t1268: F, t12724: F, t12725: F, t12728: F, t12734: F, t12739: F, t1458: F, t2314: F, t2363: F, t4028: F, t4072: F, t5113: F, t671: F, t9348: F, t89: F, t12545: F, t12550: F, t12557: F, t1442: F, t1459: F, t1849: F, t2323: F, t2364: F, t3652: F, t3660: F, t4034: F, t4037: F, t4073: F, t574: F, t652: F, t672: F, t510: F, t4098: F, t751: F, t2752: F, t4303: F, t172: F, t4095: F, t763: F, t1472: F, t2517: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12758, t12761, t12771, t12774, t12775, t12778) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1440::<F>(t12757, t666, t2358, t4043, t1444, t2342, t9384, t2341, t92, t2219, t659, t2248, t4049);
        let (t12781, t12784, t12792, t12795, t12796, t12799) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1441::<F>(t584, t95, t16, t4053, t1449, t2350, t9398, t100, t2349, t2219, t662, t2354, t4059);
        let t12808 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1442::<F>(t103, t584, t16, t4063, t100, t12771, t12774, t12775, t12778, t12781, t12784, t12792, t12795, t12796, t12799, t1445, t1447, t2336, t2351, t2355, t4050, t4054, t657, t92);
        let t12812 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1443::<F>(t12808, t656, t12747, t12750, t12752, t12754, t12758, t12761, t64, t9358, t9359, t9361, t9363);
        let t12813 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1444::<F>(t109, t12812);
        let t12816 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1445::<F>(t1268, t12724, t12725, t12728, t12734, t12739, t12813, t1458, t2314, t2363, t4028, t4072, t5113, t671, t9348);
        let t12823 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1446::<F>(t2363, t89);
        let t12832 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1447::<F>(t12545, t12550, t12557, t12725, t12734, t12816, t12823, t1442, t1459, t1849, t2314, t2323, t2364, t3652, t3660, t4028, t4034, t4037, t4073, t574, t652, t672, t9348);
        let (t12835, t12841, t12850, t12854, t12860, t12861) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1448::<F>(t12813, t510, t1458, t3652, t4098, t751, t2752, t4303, t172, t4095, t763, t1472, t2517);
    (t12808, t12813, t12816, t12823, t12832, t12835, t12841, t12850, t12854, t12860, t12861)
}
