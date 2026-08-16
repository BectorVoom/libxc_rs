//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta370 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1403;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1404;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1405;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1406;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1407;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1408;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta370(t14328: f64, t932: f64, t4446: f64, t942: f64, t1573: f64, t2929: f64, t13716: f64, t951: f64, t13563: f64, t13566: f64, t13602: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10608: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13598: f64, t13613: f64, t324: f64, t2924: f64, t4475: f64, t10632: f64, t1580: f64, t2906: f64, t10756: f64, t10820: f64, t13729: f64, t14257: f64, t1581: f64, t2856: f64, t2900: f64, t2925: f64, t2930: f64, t2933: f64, t4434: f64, t4449: f64, t4472: f64, t924: f64, t943: f64, t952: f64, t10817: f64, t4359: f64, t10655: f64, t4400: f64, t4396: f64, t912: f64, t2792: f64, t1557: f64, t2836: f64, t2793: f64, t4399: f64, t10661: f64, t2844: f64, t4395: f64, t2842: f64, t10704: f64, t1556: f64, t10702: f64, t10832: f64, t931: f64, t10740: f64, t10765: f64, t2861: f64, t311: f64, t4416: f64, t4438: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14329, t14332, t14337, t14344, t14363) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1403(t14328, t932, t4446, t942, t1573, t2929, t13716, t951, t13563, t13566, t13602, t10556, t10558, t10560, t10562, t10608, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613);
        let (t14364, t14373) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1404(t14363, t324, t2924, t4475, t10632, t1580, t2906, t10756, t10820, t13729, t14257, t14329, t14332, t14337, t14344, t1581, t2856, t2900, t2925, t2930, t2933, t4434, t4449, t4472, t924, t943, t952);
        let (t14376, t14378, t14381, t14384, t14387) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1405(t10817, t4359, t10655, t4400, t4396, t912, t2792, t1557, t2836, t2793, t4399, t10661);
        let (t14391, t14394, t14398, t14409, t14410) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1406(t2844, t4395, t912, t2842, t2836, t4399, t10704, t1556, t2793, t10702, t13566, t13602);
        let t14419 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1407(t10556, t10558, t10560, t10562, t10832, t13563, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613, t14409, t14410);
        let (t14424, t14428) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1408(t1557, t2793, t2842, t4434, t931, t10740, t10765, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14419, t2861, t311, t4416, t4438);
    (t14364, t14373, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14424, t14428)
}
