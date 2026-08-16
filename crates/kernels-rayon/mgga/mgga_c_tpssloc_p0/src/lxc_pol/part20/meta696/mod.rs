//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta696 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2654;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2655;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2656;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2657;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2658;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2659;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2660;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta696(t12283: f64, t16271: f64, t16383: f64, t16370: f64, t16060: f64, t3798: f64, t1354: f64, t12345: f64, t5310: f64, t12339: f64, t16150: f64, t3866: f64, t12240: f64, t12379: f64, t12392: f64, t12409: f64, t16242: f64, t16261: f64, t16394: f64, t16401: f64, t3803: f64, t40000: f64, t40168: f64, t40169: f64, t5235: f64, t5246: f64, t5248: f64, t5249: f64, t16155: f64, t1827: f64, t40123: f64, t1824: f64, t3850: f64, t3802: f64, t1799: f64, t1340: f64, t53909: f64, t12255: f64, t12305: f64, t12336: f64, t1307: f64, t1363: f64, t16018: f64, t16217: f64, t16224: f64, t16225: f64, t16305: f64, t16306: f64, t3783: f64, t3807: f64, t3809: f64, t3851: f64, t3870: f64, t5240: f64, t54013: f64, t820: f64, t39947: f64, t16314: f64, t16398: f64, t16387: f64, t12251: f64, t12297: f64, t12351: f64, t12404: f64, t16233: f64, t16278: f64, t16285: f64, t3734: f64, t3853: f64, t40006: f64, t40008: f64, t40012: f64, t40019: f64, t40022: f64, t5187: f64, t40138: f64, t5303: f64, t16366: f64, t16308: f64, t1352: f64, t16153: f64, t16311: f64, t3805: f64, t3856: f64, t40052: f64, t40054: f64, t40060: f64, t40065: f64, t40079: f64, t40081: f64, t40083: f64, t40178: f64, t54015: f64, t3791: f64, t12168: f64, t12369: f64, t16364: f64, t40089: f64, t40114: f64, t40116: f64, t40124: f64, t40126: f64, t40128: f64, t40131: f64, t40139: f64, t40145: f64, t53958: f64, t54068: f64, t12300: f64, t5289: f64, t16208: f64, t3799: f64, t39249: f64, t39256: f64, t39261: f64, t39266: f64, t39304: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t53778: f64, t53780: f64, t53783: f64, t53788: f64, t53797: f64, t53799: f64, t53800: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54114, t54116, t54118, t54125, t54132, t54133, t54135) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2654(t12283, t16271, t16383, t16370, t16060, t3798, t1354, t12345, t5310, t12339, t16150, t3866);
        let t54137 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2655(t12240, t12379, t12392, t12409, t16242, t16261, t16394, t16401, t3803, t40000, t40168, t40169, t5235, t5246, t5248, t5249, t54114, t54116, t54118, t54125, t54132, t54133, t54135);
        let t54183 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2656(t16155, t3866, t1827, t40123, t1824, t3850, t16060, t3802, t1799, t1340, t53909, t12255, t12305, t12336, t1307, t1354, t1363, t16018, t16150, t16217, t16224, t16225, t16305, t16306, t3783, t3803, t3807, t3809, t3851, t3870, t5240, t5246, t5248, t5249, t5310, t54013, t820);
        let t54215 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2657(t16217, t3866, t1827, t39947, t16314, t16398, t16387, t12251, t12297, t12351, t12404, t1363, t16233, t16278, t16285, t16394, t3734, t3853, t40006, t40008, t40012, t40019, t40022, t5187, t5248, t5249, t820);
        let t54245 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2658(t40138, t5303, t12283, t16366, t16308, t1352, t16153, t16224, t16242, t16311, t3803, t3805, t3856, t40052, t40054, t40060, t40065, t40079, t40081, t40083, t40178, t5246, t5248, t5249, t54013, t54015);
        let (t54258, t54277) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2659(t1824, t3791, t12168, t12369, t1352, t16224, t16305, t16364, t3803, t3805, t3851, t40089, t40114, t40116, t40124, t40126, t40128, t40131, t40139, t40145, t5246, t5248, t5249, t53958, t54068);
        let (t54284, t54293, t54295, t54311) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2660(t1307, t16153, t12300, t5289, t16208, t3799, t39249, t39256, t39261, t39266, t39304, t39309, t39312, t39316, t39320, t53778, t53780, t53783, t53788, t53797, t53799, t53800);
    (t54137, t54183, t54215, t54245, t54258, t54277, t54284, t54293, t54295, t54311)
}
