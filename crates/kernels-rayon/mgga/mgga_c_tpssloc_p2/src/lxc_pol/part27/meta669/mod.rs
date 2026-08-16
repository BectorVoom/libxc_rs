//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta669 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2363;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2364;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2365;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2366;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2367;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2368;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2369;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2370;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2371;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta669(t1983: f64, t2019: f64, t55169: f64, t510: f64, t652: f64, t86604: f64, t26114: f64, t6535: f64, t26179: f64, t2314: f64, t25994: f64, t12823: f64, t7461: f64, t12550: f64, t1442: f64, t22461: f64, t22619: f64, t23829: f64, t26103: f64, t4028: f64, t4073: f64, t6517: f64, t7472: f64, t90351: f64, t91713: f64, t91715: f64, t91718: f64, t91722: f64, t91724: f64, t91726: f64, t9348: f64, t25980: f64, t4034: f64, t12813: f64, t89: f64, t1874: f64, t6525: f64, t22561: f64, t7458: f64, t3652: f64, t7467: f64, t22579: f64, t7685: f64, t55934: f64, t12725: f64, t26168: f64, t6876: f64, t25989: f64, t83886: f64, t15857: f64, t1873: f64, t45632: f64, t12841: f64, t1774: f64, t1849: f64, t22559: f64, t2320: f64, t23855: f64, t4037: f64, t7670: f64, t90352: f64, t90030: f64, t90422: f64, t91574: f64, t91617: f64, t91663: f64, t91709: f64, t26135: f64, t3941: f64, t671: f64, t2363: f64, t2022: f64, t2319: f64, t1458: f64, t16538: f64, t16541: f64, t23877: f64, t23880: f64, t26523: f64, t4072: f64, t5376: f64, t577: f64, t83980: f64, t86642: f64, t86646: f64, t86647: f64, t86651: f64, t86653: f64, t86655: f64, t86656: f64, t86660: f64, t86668: f64, t5381: f64, t26509: f64, t580: f64, t1404: f64, t7758: f64, t1395: f64, t7774: f64, t1396: f64, t1398: f64, t26510: f64, t26555: f64, t3: f64, t3932: f64, t5364: f64, t7020: f64, t80599: f64, t80601: f64, t80605: f64, t86640: f64, t86580: f64) -> f64 {
        let (t91730, t91735, t91737, t91739, t91747, t91749) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2363(t1983, t2019, t55169, t510, t652, t86604, t26114, t6535, t26179, t2314, t25994, t12823, t7461);
        let t91750 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2364(t12550, t1442, t22461, t22619, t23829, t26103, t4028, t4073, t510, t6517, t7472, t90351, t91713, t91715, t91718, t91722, t91724, t91726, t91730, t91735, t91737, t91739, t91747, t91749, t9348);
        let (t91752, t91755, t91757, t91759, t91762) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2365(t25980, t4034, t12813, t89, t1874, t26179, t6525, t22561, t7458, t3652, t652, t7467);
        let (t91763, t91765, t91767, t91769, t91771, t91777) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2366(t22579, t7685, t1874, t55934, t12725, t6525, t26168, t6876, t25989, t83886, t25994, t4034);
        let t91789 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2367(t15857, t1873, t652, t1874, t45632, t12841, t1774, t1849, t22461, t22559, t2320, t23855, t4037, t510, t6517, t7670, t90352, t91752, t91755, t91757, t91759, t91762, t91763, t91765, t91767, t91769, t91771, t91777);
        let (t91792, t91799, t91802) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2368(t90030, t90422, t91574, t91617, t91663, t91709, t91750, t91789, t26135, t3941, t671, t2363, t7467);
        let t91806 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2369(t2022, t2319, t1458, t16538, t16541, t2363, t23877, t23880, t26523, t4072, t5376, t577, t671, t83980, t86642, t86646, t86647, t86651, t86653, t86655, t86656, t86660, t86668, t91792, t91799, t91802);
        let t91827 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2370(t2022, t5381, t26509, t580, t1404, t7758, t1395, t7774, t1396, t1398, t26510, t26555, t3, t3932, t5364, t7020, t80599, t80601, t80605, t86640, t91792, t91806);
        let tv4rho3sigma3 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2371(t86580, t91827);
    tv4rho3sigma3
}
