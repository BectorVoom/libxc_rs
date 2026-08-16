//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1546;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1547;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1548;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1549;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1550;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1551;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta382<F: Float>(t14328: F, t932: F, t4446: F, t942: F, t1573: F, t2929: F, t13716: F, t951: F, t13563: F, t13566: F, t13602: F, t10556: F, t10558: F, t10560: F, t10562: F, t10608: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13613: F, t324: F, t2924: F, t4475: F, t10632: F, t1580: F, t2906: F, t10756: F, t10820: F, t13729: F, t14257: F, t1581: F, t2856: F, t2900: F, t2925: F, t2930: F, t2933: F, t4434: F, t4449: F, t4472: F, t924: F, t943: F, t952: F, t10817: F, t4359: F, t10655: F, t4400: F, t4396: F, t912: F, t2792: F, t1557: F, t2836: F, t2793: F, t4399: F, t10661: F, t2844: F, t4395: F, t2842: F, t10704: F, t1556: F, t10702: F, t10832: F, t931: F, t10740: F, t10765: F, t2861: F, t311: F, t4416: F, t4438: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14329, t14332, t14337, t14344, t14363) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1546::<F>(t14328, t932, t4446, t942, t1573, t2929, t13716, t951, t13563, t13566, t13602, t10556, t10558, t10560, t10562, t10608, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613);
        let (t14364, t14373) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1547::<F>(t14363, t324, t2924, t4475, t10632, t1580, t2906, t10756, t10820, t13729, t14257, t14329, t14332, t14337, t14344, t1581, t2856, t2900, t2925, t2930, t2933, t4434, t4449, t4472, t924, t943, t952);
        let (t14376, t14378, t14381, t14384, t14387) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1548::<F>(t10817, t4359, t10655, t4400, t4396, t912, t2792, t1557, t2836, t2793, t4399, t10661);
        let (t14391, t14394, t14398, t14409, t14410) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1549::<F>(t2844, t4395, t912, t2842, t2836, t4399, t10704, t1556, t2793, t10702, t13566, t13602);
        let t14419 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1550::<F>(t10556, t10558, t10560, t10562, t10832, t13563, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613, t14409, t14410);
        let (t14424, t14428) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1551::<F>(t1557, t2793, t2842, t4434, t931, t10740, t10765, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14419, t2861, t311, t4416, t4438);
    (t14364, t14373, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14424, t14428)
}
