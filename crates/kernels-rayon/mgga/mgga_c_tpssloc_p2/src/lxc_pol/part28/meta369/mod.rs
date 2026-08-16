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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta369(t14142: f64, t4582: f64, t12648: f64, t4583: f64, t13559: f64, t977: f64, t2960: f64, t4603: f64, t1606: f64, t698: f64, t973: f64, t1043: f64, t2770: f64, t1409: f64, t2244: f64, t10263: f64, t10403: f64, t1041: f64, t10413: f64, t10896: f64, t14122: f64, t14126: f64, t14130: f64, t14136: f64, t14139: f64, t1607: f64, t3070: f64, t3117: f64, t4562: f64, t4565: f64, t4585: f64, t10277: f64, t3061: f64, t12652: f64, t4588: f64, t10216: f64, t10969: f64, t135: f64, t4608: f64, t12606: f64, t998: f64, t974: f64, t10868: f64, t1539: f64, t248: f64, t1009: f64, t4552: f64, t1011: f64, t1019: f64, t1615: f64, t3131: f64, t1022: f64, t883: f64, t607: f64, t3071: f64, t360: f64, t4342: f64, t1025: f64, t10909: f64, t10923: f64, t10927: f64, t4590: f64, t4609: f64, t4337: f64, t10408: f64, t13510: f64, t13512: f64, t13514: f64, t13517: f64, t13519: f64, t13522: f64, t13524: f64, t13526: f64, t13657: f64, t13661: f64, t13665: f64, t13720: f64, t13722: f64, t13726: f64, t13729: f64, t13731: f64, t13734: f64, t13566: f64, t13602: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10636: f64, t13563: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13598: f64, t13613: f64, t291: f64, t10629: f64, t1580: f64, t10632: f64, t2906: f64, t959: f64, t1573: f64, t2904: f64, t4408: f64, t923: f64, t1561: f64, t2885: f64, t2860: f64, t10760: f64, t1569: f64, t2863: f64, t2881: f64, t2889: f64, t2907: f64, t4411: f64, t933: f64, t13550: f64, t10296: f64, t10298: f64, t10302: f64, t13644: f64, t13630: f64, t13632: f64, t13635: f64, t13638: f64, t13640: f64, t13642: f64, t13647: f64, t10300: f64, t10784: f64, t10785: f64, t13530: f64, t13534: f64, t13539: f64, t13544: f64, t13548: f64, t13552: f64, t13557: f64, t13561: f64, t13616: f64, t13624: f64, t13626: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14143, t14147, t14152, t14158, t14160, t14164) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1391(t14142, t4582, t12648, t4583, t13559, t977, t2960, t4603, t1606, t698, t973, t1043, t2770);
        let t14165 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1392(t1409, t2244);
        let t14170 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1393(t14164, t14165, t4582, t10263, t10403, t1041, t10413, t10896, t14122, t14126, t14130, t14136, t14139, t14143, t14147, t14152, t14158, t14160, t1607, t2960, t3070, t3117, t4562, t4565, t4585, t973);
        let (t14174, t14180, t14184, t14189, t14194) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1394(t10277, t3061, t14165, t4582, t12652, t4588, t12648, t10216, t10969, t135, t4608, t973);
        let (t14198, t14203, t14205, t14207, t14211) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1395(t12606, t998, t974, t10868, t1539, t248, t1041, t1009, t4552, t1011, t1019, t1615, t3131);
        let (t14228, t14233) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1396(t1022, t883, t607, t14211, t3071, t1615, t360, t4342, t1025, t10403, t1041, t10413, t10909, t10923, t10927, t14174, t14180, t14184, t14189, t14194, t14198, t14203, t14207, t2960, t3070, t3117, t4590, t4609, t973);
        let (t14235, t14238) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1397(t14228, t4337, t10408, t13510, t13512, t13514, t13517, t13519, t13522, t13524, t13526, t13657, t13661, t13665, t13720, t13722, t13726, t13729, t13731, t13734);
        let t14255 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1398(t13566, t13602, t10556, t10558, t10560, t10562, t10636, t13563, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613);
        let (t14257, t14262, t14263, t14266, t14271) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1399(t14255, t291, t10629, t1580, t10632, t2906, t959, t1573, t2904, t4408, t923, t1561, t2885);
        let t14279 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1400(t1561, t2860, t10760, t13517, t13519, t13522, t13524, t13526, t13657, t14263, t14266, t14271, t1569, t2863, t2881, t2889, t2907, t4411, t933);
        let (t14287, t14291, t14304) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1401(t13550, t13563, t10296, t10298, t10302, t13566, t13569, t13572, t13575, t13578, t13581, t13584, t13587);
        let t14328 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1402(t13644, t13602, t13598, t13613, t13630, t13632, t13635, t13638, t13640, t13642, t13647, t10300, t10556, t10558, t10560, t10562, t10784, t10785, t13530, t13534, t13539, t13544, t13548, t13552, t13557, t13561, t13616, t13624, t13626, t14287, t14291, t14304);
    (t14165, t14170, t14205, t14233, t14235, t14238, t14257, t14262, t14279, t14328)
}
