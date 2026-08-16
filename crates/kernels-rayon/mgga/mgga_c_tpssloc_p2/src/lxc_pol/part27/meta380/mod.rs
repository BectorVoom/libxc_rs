//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta380 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1556;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1557;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1558;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1559;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1560;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1561;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1562;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1563;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta380(t14228: f64, t4337: f64, t10408: f64, t13510: f64, t13512: f64, t13514: f64, t13517: f64, t13519: f64, t13522: f64, t13524: f64, t13526: f64, t13657: f64, t13661: f64, t13665: f64, t13720: f64, t13722: f64, t13726: f64, t13729: f64, t13731: f64, t13734: f64, t13566: f64, t13602: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10636: f64, t13563: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13598: f64, t13613: f64, t291: f64, t10629: f64, t1580: f64, t10632: f64, t2906: f64, t959: f64, t1573: f64, t2904: f64, t4408: f64, t923: f64, t1561: f64, t2885: f64, t2860: f64, t10760: f64, t1569: f64, t2863: f64, t2881: f64, t2889: f64, t2907: f64, t4411: f64, t933: f64, t13550: f64, t10296: f64, t10298: f64, t10302: f64, t13644: f64, t13630: f64, t13632: f64, t13635: f64, t13638: f64, t13640: f64, t13642: f64, t13647: f64, t10300: f64, t10784: f64, t10785: f64, t13530: f64, t13534: f64, t13539: f64, t13544: f64, t13548: f64, t13552: f64, t13557: f64, t13561: f64, t13616: f64, t13624: f64, t13626: f64, t932: f64, t4446: f64, t942: f64, t2929: f64, t13716: f64, t951: f64, t10608: f64, t324: f64, t2924: f64, t4475: f64, t10756: f64, t10820: f64, t1581: f64, t2856: f64, t2900: f64, t2925: f64, t2930: f64, t2933: f64, t4434: f64, t4449: f64, t4472: f64, t924: f64, t943: f64, t952: f64, t10817: f64, t4359: f64, t10655: f64, t4400: f64, t4396: f64, t912: f64, t2792: f64, t1557: f64, t2836: f64, t2793: f64, t4399: f64, t10661: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14235, t14238) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1556(t14228, t4337, t10408, t13510, t13512, t13514, t13517, t13519, t13522, t13524, t13526, t13657, t13661, t13665, t13720, t13722, t13726, t13729, t13731, t13734);
        let t14255 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1557(t13566, t13602, t10556, t10558, t10560, t10562, t10636, t13563, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613);
        let (t14257, t14262, t14263, t14266, t14271) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1558(t14255, t291, t10629, t1580, t10632, t2906, t959, t1573, t2904, t4408, t923, t1561, t2885);
        let t14279 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1559(t1561, t2860, t10760, t13517, t13519, t13522, t13524, t13526, t13657, t14263, t14266, t14271, t1569, t2863, t2881, t2889, t2907, t4411, t933);
        let (t14287, t14291, t14304) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1560(t13550, t13563, t10296, t10298, t10302, t13566, t13569, t13572, t13575, t13578, t13581, t13584, t13587);
        let t14328 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1561(t13644, t13602, t13598, t13613, t13630, t13632, t13635, t13638, t13640, t13642, t13647, t10300, t10556, t10558, t10560, t10562, t10784, t10785, t13530, t13534, t13539, t13544, t13548, t13552, t13557, t13561, t13616, t13624, t13626, t14287, t14291, t14304);
        let (t14329, t14332, t14337, t14344, t14363) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1562(t14328, t932, t4446, t942, t1573, t2929, t13716, t951, t13563, t13566, t13602, t10556, t10558, t10560, t10562, t10608, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613);
        let (t14364, t14373) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1563(t14363, t324, t2924, t4475, t10632, t1580, t2906, t10756, t10820, t13729, t14257, t14329, t14332, t14337, t14344, t1581, t2856, t2900, t2925, t2930, t2933, t4434, t4449, t4472, t924, t943, t952);
        let (t14376, t14378, t14381, t14384, t14387) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1564(t10817, t4359, t10655, t4400, t4396, t912, t2792, t1557, t2836, t2793, t4399, t10661);
    (t14235, t14238, t14257, t14262, t14279, t14364, t14373, t14376, t14378, t14381, t14384, t14387)
}
