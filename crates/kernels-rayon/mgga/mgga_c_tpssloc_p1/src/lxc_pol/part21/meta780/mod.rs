//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta780 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2706;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2707;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2708;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2709;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2710;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta780(t19815: f64, t3789: f64, t40159: f64, t6390: f64, t236: f64, t240: f64, t3869: f64, t247: f64, t5249: f64, t3798: f64, t1354: f64, t40130: f64, t1827: f64, t54532: f64, t16232: f64, t5234: f64, t12419: f64, t12429: f64, t1363: f64, t16208: f64, t16226: f64, t16235: f64, t16278: f64, t16312: f64, t19855: f64, t19871: f64, t19962: f64, t20468: f64, t3719: f64, t3734: f64, t3795: f64, t3853: f64, t3870: f64, t39978: f64, t40065: f64, t40070: f64, t40079: f64, t5235: f64, t5246: f64, t5289: f64, t5334: f64, t5344: f64, t54178: f64, t6330: f64, t6347: f64, t820: f64, t1307: f64, t5286: f64, t1351: f64, t6387: f64, t12283: f64, t19894: f64, t12240: f64, t1352: f64, t16224: f64, t16233: f64, t16271: f64, t16275: f64, t16305: f64, t16394: f64, t1825: f64, t19956: f64, t19994: f64, t210: f64, t3733: f64, t3803: f64, t3807: f64, t40124: f64, t40126: f64, t40145: f64, t5248: f64, t54014: f64, t54068: f64, t54153: f64, t54293: f64, t54295: f64, t54533: f64, t54535: f64, t6374: f64, t6394: f64, t19981: f64, t19986: f64, t16205: f64, t3792: f64, t19823: f64, t40021: f64, t12211: f64, t19827: f64, t19831: f64, t39249: f64, t39256: f64, t39261: f64, t39266: f64, t39304: f64, t39309: f64, t39312: f64, t56092: f64, t56093: f64, t56094: f64, t56098: f64, t56100: f64, t56103: f64, t56105: f64, t56114: f64, t56115: f64, t56119: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57033, t57041, t57043, t57044, t57046, t57057, t57071) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2706(t19815, t3789, t40159, t6390, t236, t240, t3869, t247, t5249, t3798, t1354, t40130);
        let t57084 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2707(t1827, t54532, t16232, t5234, t12419, t12429, t1363, t16208, t16226, t16235, t16278, t16312, t19855, t19871, t19962, t20468, t3719, t3734, t3795, t3853, t3870, t39978, t40065, t40070, t40079, t5235, t5246, t5289, t5334, t5344, t54178, t57033, t57041, t57044, t57046, t57057, t57071, t6330, t6347, t820);
        let (t57086, t57133) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2708(t1307, t5286, t1351, t6387, t12283, t19894, t12240, t1352, t16224, t16233, t16271, t16275, t16305, t16394, t1825, t19871, t19956, t19994, t210, t3719, t3733, t3803, t3807, t40124, t40126, t40145, t5246, t5248, t54014, t54068, t54153, t54293, t54295, t54533, t54535, t6374, t6394);
        let (t57143, t57145, t57147, t57158, t57160, t57170, t57172) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2709(t12283, t19981, t19986, t16205, t3792, t19823, t40021, t12211, t19827, t19831, t1351, t6330);
        let t57193 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2710(t39249, t39256, t39261, t39266, t39304, t39309, t39312, t56092, t56093, t56094, t56098, t56100, t56103, t56105, t56114, t56115, t56119);
    (t57043, t57084, t57086, t57133, t57143, t57145, t57147, t57158, t57160, t57170, t57172, t57193)
}
