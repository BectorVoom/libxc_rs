//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta780 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2706;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2707;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2708;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2709;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2710;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta780<F: Float>(t19815: F, t3789: F, t40159: F, t6390: F, t236: F, t240: F, t3869: F, t247: F, t5249: F, t3798: F, t1354: F, t40130: F, t1827: F, t54532: F, t16232: F, t5234: F, t12419: F, t12429: F, t1363: F, t16208: F, t16226: F, t16235: F, t16278: F, t16312: F, t19855: F, t19871: F, t19962: F, t20468: F, t3719: F, t3734: F, t3795: F, t3853: F, t3870: F, t39978: F, t40065: F, t40070: F, t40079: F, t5235: F, t5246: F, t5289: F, t5334: F, t5344: F, t54178: F, t6330: F, t6347: F, t820: F, t1307: F, t5286: F, t1351: F, t6387: F, t12283: F, t19894: F, t12240: F, t1352: F, t16224: F, t16233: F, t16271: F, t16275: F, t16305: F, t16394: F, t1825: F, t19956: F, t19994: F, t210: F, t3733: F, t3803: F, t3807: F, t40124: F, t40126: F, t40145: F, t5248: F, t54014: F, t54068: F, t54153: F, t54293: F, t54295: F, t54533: F, t54535: F, t6374: F, t6394: F, t19981: F, t19986: F, t16205: F, t3792: F, t19823: F, t40021: F, t12211: F, t19827: F, t19831: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t39309: F, t39312: F, t56092: F, t56093: F, t56094: F, t56098: F, t56100: F, t56103: F, t56105: F, t56114: F, t56115: F, t56119: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t57033, t57041, t57043, t57044, t57046, t57057, t57071) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2706::<F>(t19815, t3789, t40159, t6390, t236, t240, t3869, t247, t5249, t3798, t1354, t40130);
        let t57084 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2707::<F>(t1827, t54532, t16232, t5234, t12419, t12429, t1363, t16208, t16226, t16235, t16278, t16312, t19855, t19871, t19962, t20468, t3719, t3734, t3795, t3853, t3870, t39978, t40065, t40070, t40079, t5235, t5246, t5289, t5334, t5344, t54178, t57033, t57041, t57044, t57046, t57057, t57071, t6330, t6347, t820);
        let (t57086, t57133) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2708::<F>(t1307, t5286, t1351, t6387, t12283, t19894, t12240, t1352, t16224, t16233, t16271, t16275, t16305, t16394, t1825, t19871, t19956, t19994, t210, t3719, t3733, t3803, t3807, t40124, t40126, t40145, t5246, t5248, t54014, t54068, t54153, t54293, t54295, t54533, t54535, t6374, t6394);
        let (t57143, t57145, t57147, t57158, t57160, t57170, t57172) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2709::<F>(t12283, t19981, t19986, t16205, t3792, t19823, t40021, t12211, t19827, t19831, t1351, t6330);
        let t57193 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2710::<F>(t39249, t39256, t39261, t39266, t39304, t39309, t39312, t56092, t56093, t56094, t56098, t56100, t56103, t56105, t56114, t56115, t56119);
    (t57043, t57084, t57086, t57133, t57143, t57145, t57147, t57158, t57160, t57170, t57172, t57193)
}
