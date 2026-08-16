//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta369 (260520-c91 hierarchical CSE).
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
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1391;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1392;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1393;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1394;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1395;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1396;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1397;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1398;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1399;
use chunk9::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1400;
use chunk10::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1401;
use chunk11::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1402;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta369<F: Float>(t14142: F, t4582: F, t12648: F, t4583: F, t13559: F, t977: F, t2960: F, t4603: F, t1606: F, t698: F, t973: F, t1043: F, t2770: F, t1409: F, t2244: F, t10263: F, t10403: F, t1041: F, t10413: F, t10896: F, t14122: F, t14126: F, t14130: F, t14136: F, t14139: F, t1607: F, t3070: F, t3117: F, t4562: F, t4565: F, t4585: F, t10277: F, t3061: F, t12652: F, t4588: F, t10216: F, t10969: F, t135: F, t4608: F, t12606: F, t998: F, t974: F, t10868: F, t1539: F, t248: F, t1009: F, t4552: F, t1011: F, t1019: F, t1615: F, t3131: F, t1022: F, t883: F, t607: F, t3071: F, t360: F, t4342: F, t1025: F, t10909: F, t10923: F, t10927: F, t4590: F, t4609: F, t4337: F, t10408: F, t13510: F, t13512: F, t13514: F, t13517: F, t13519: F, t13522: F, t13524: F, t13526: F, t13657: F, t13661: F, t13665: F, t13720: F, t13722: F, t13726: F, t13729: F, t13731: F, t13734: F, t13566: F, t13602: F, t10556: F, t10558: F, t10560: F, t10562: F, t10636: F, t13563: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13613: F, t291: F, t10629: F, t1580: F, t10632: F, t2906: F, t959: F, t1573: F, t2904: F, t4408: F, t923: F, t1561: F, t2885: F, t2860: F, t10760: F, t1569: F, t2863: F, t2881: F, t2889: F, t2907: F, t4411: F, t933: F, t13550: F, t10296: F, t10298: F, t10302: F, t13644: F, t13630: F, t13632: F, t13635: F, t13638: F, t13640: F, t13642: F, t13647: F, t10300: F, t10784: F, t10785: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13552: F, t13557: F, t13561: F, t13616: F, t13624: F, t13626: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14143, t14147, t14152, t14158, t14160, t14164) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1391::<F>(t14142, t4582, t12648, t4583, t13559, t977, t2960, t4603, t1606, t698, t973, t1043, t2770);
        let t14165 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1392::<F>(t1409, t2244);
        let t14170 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1393::<F>(t14164, t14165, t4582, t10263, t10403, t1041, t10413, t10896, t14122, t14126, t14130, t14136, t14139, t14143, t14147, t14152, t14158, t14160, t1607, t2960, t3070, t3117, t4562, t4565, t4585, t973);
        let (t14174, t14180, t14184, t14189, t14194) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1394::<F>(t10277, t3061, t14165, t4582, t12652, t4588, t12648, t10216, t10969, t135, t4608, t973);
        let (t14198, t14203, t14205, t14207, t14211) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1395::<F>(t12606, t998, t974, t10868, t1539, t248, t1041, t1009, t4552, t1011, t1019, t1615, t3131);
        let (t14228, t14233) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1396::<F>(t1022, t883, t607, t14211, t3071, t1615, t360, t4342, t1025, t10403, t1041, t10413, t10909, t10923, t10927, t14174, t14180, t14184, t14189, t14194, t14198, t14203, t14207, t2960, t3070, t3117, t4590, t4609, t973);
        let (t14235, t14238) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1397::<F>(t14228, t4337, t10408, t13510, t13512, t13514, t13517, t13519, t13522, t13524, t13526, t13657, t13661, t13665, t13720, t13722, t13726, t13729, t13731, t13734);
        let t14255 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1398::<F>(t13566, t13602, t10556, t10558, t10560, t10562, t10636, t13563, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613);
        let (t14257, t14262, t14263, t14266, t14271) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1399::<F>(t14255, t291, t10629, t1580, t10632, t2906, t959, t1573, t2904, t4408, t923, t1561, t2885);
        let t14279 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1400::<F>(t1561, t2860, t10760, t13517, t13519, t13522, t13524, t13526, t13657, t14263, t14266, t14271, t1569, t2863, t2881, t2889, t2907, t4411, t933);
        let (t14287, t14291, t14304) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1401::<F>(t13550, t13563, t10296, t10298, t10302, t13566, t13569, t13572, t13575, t13578, t13581, t13584, t13587);
        let t14328 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1402::<F>(t13644, t13602, t13598, t13613, t13630, t13632, t13635, t13638, t13640, t13642, t13647, t10300, t10556, t10558, t10560, t10562, t10784, t10785, t13530, t13534, t13539, t13544, t13548, t13552, t13557, t13561, t13616, t13624, t13626, t14287, t14291, t14304);
    (t14165, t14170, t14205, t14233, t14235, t14238, t14257, t14262, t14279, t14328)
}
