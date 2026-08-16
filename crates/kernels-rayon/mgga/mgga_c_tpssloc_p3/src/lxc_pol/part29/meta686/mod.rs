//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta686 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2350;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2351;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2352;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2353;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2354;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2355;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2356;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta686(t12823: f64, t15857: f64, t2114: f64, t2312: f64, t2314: f64, t2323: f64, t27290: f64, t27858: f64, t27863: f64, t27879: f64, t4034: f64, t5107: f64, t5361: f64, t574: f64, t652: f64, t671: f64, t672: f64, t7264: f64, t7412: f64, t7989: f64, t8103: f64, t91763: f64, t91765: f64, t91767: f64, t91769: f64, t91771: f64, t91777: f64, t91780: f64, t91782: f64, t96238: f64, t96269: f64, t96271: f64, t94223: f64, t94236: f64, t94257: f64, t94272: f64, t95970: f64, t96228: f64, t96232: f64, t2174: f64, t5363: f64, t1404: f64, t8110: f64, t1851: f64, t7426: f64, t27907: f64, t580: f64, t2169: f64, t5381: f64, t16507: f64, t16546: f64, t2170: f64, t3: f64, t3932: f64, t3946: f64, t5364: f64, t7416: f64, t8111: f64, t8119: f64, t85405: f64, t1395: f64, t1858: f64, t7415: f64, t27930: f64, t576: f64, t112: f64, t2319: f64, t1458: f64, t16538: f64, t2363: f64, t24969: f64, t24972: f64, t27921: f64, t4072: f64, t577: f64, t85423: f64, t86582: f64, t86606: f64, t86610: f64, t86612: f64, t86614: f64, t86616: f64, t86619: f64, t86622: f64, t86625: f64, t86629: f64, t111: f64, t12813: f64, t16541: f64, t5376: f64, t7423: f64, t85416: f64, t86631: f64, t86633: f64, t86635: f64, t86637: f64, t86639: f64, t86642: f64, t86646: f64, t86651: f64, t86653: f64, t86655: f64, t86660: f64, t86668: f64, t91799: f64, t91802: f64, t1396: f64, t1398: f64, t1852: f64, t24955: f64, t24977: f64, t27908: f64, t85403: f64, t85407: f64, t85412: f64, t86557: f64, t86559: f64) -> f64 {
        let t96274 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2350(t12823, t15857, t2114, t2312, t2314, t2323, t27290, t27858, t27863, t27879, t4034, t5107, t5361, t574, t652, t671, t672, t7264, t7412, t7989, t8103, t91763, t91765, t91767, t91769, t91771, t91777, t91780, t91782, t96238, t96269, t96271);
        let (t96277, t96281, t96283) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2351(t94223, t94236, t94257, t94272, t95970, t96228, t96232, t96274, t2174, t5363, t1404, t8110);
        let t96297 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2352(t1851, t7426, t27907, t580, t2169, t5381, t16507, t16546, t2170, t2174, t3, t3932, t3946, t5364, t7416, t8111, t8119, t85405, t96277, t96281, t96283);
        let (t96300, t96303, t96308, t96327) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2353(t1395, t8119, t1858, t7415, t27930, t576, t112, t27907, t2169, t2319, t1458, t16538, t2363, t24969, t24972, t27921, t4072, t577, t671, t85423, t86582, t86606, t86610, t86612, t86614, t86616, t86619, t86622, t86625, t86629, t96277);
        let t96337 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2354(t111, t8110, t12813, t16541, t2319, t24972, t5376, t7423, t85416, t86631, t86633, t86635, t86637, t86639, t86642, t86646, t86651, t86653, t86655, t86660, t86668, t91799, t91802);
        let t96340 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2355(t1396, t1398, t1404, t1852, t1858, t24955, t24977, t27908, t27930, t85403, t85407, t85412, t86557, t86559, t96300, t96303, t96308, t96327, t96337);
        let tv4rho3sigma5 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2356(t96297, t96340);
    tv4rho3sigma5
}
