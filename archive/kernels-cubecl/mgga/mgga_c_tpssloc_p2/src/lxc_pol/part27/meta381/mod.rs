//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1565;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1566;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1567;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1568;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1569;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1570;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1571;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta381<F: Float>(t2844: F, t4395: F, t912: F, t2842: F, t2836: F, t4399: F, t10704: F, t1556: F, t2793: F, t10702: F, t13566: F, t13602: F, t10556: F, t10558: F, t10560: F, t10562: F, t10832: F, t13563: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13613: F, t1557: F, t4434: F, t931: F, t10740: F, t10765: F, t14376: F, t14378: F, t14381: F, t14384: F, t14387: F, t2861: F, t311: F, t4416: F, t4438: F, t1569: F, t2880: F, t2862: F, t4437: F, t2888: F, t4433: F, t10813: F, t1568: F, t4472: F, t950: F, t1581: F, t2924: F, t2906: F, t4475: F, t2932: F, t4471: F, t10747: F, t10771: F, t10811: F, t10825: F, t10828: F, t2886: F, t2905: F, t2930: F, t4454: F, t4476: F, t14279: F, t14373: F, t300: F, t4446: F, t961: F, t2948: F, t4483: F, t14364: F, t2907: F, t4496: F, t959: F, t2952: F, t10623: F, t1589: F, t14257: F, t14262: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14391, t14394, t14398, t14409, t14410) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1565::<F>(t2844, t4395, t912, t2842, t2836, t4399, t10704, t1556, t2793, t10702, t13566, t13602);
        let t14419 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1566::<F>(t10556, t10558, t10560, t10562, t10832, t13563, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613, t14409, t14410);
        let (t14424, t14428) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1567::<F>(t1557, t2793, t2842, t4434, t931, t10740, t10765, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14419, t2861, t311, t4416, t4438);
        let (t14429, t14432, t14436, t14439, t14443, t14450, t14453) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1568::<F>(t1569, t2880, t2862, t4437, t2888, t4433, t931, t10813, t1568, t4472, t950, t1581, t2924);
        let t14469 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1569::<F>(t2906, t4475, t2932, t4471, t950, t1581, t1569, t2862, t10747, t10771, t10811, t10825, t10828, t14429, t14432, t14436, t14439, t14443, t14450, t14453, t2861, t2886, t2905, t2930, t4454, t4476);
        let (t14472, t14475, t14477, t14479, t14480) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1570::<F>(t14279, t14373, t14428, t14469, t300, t4446, t961, t2948, t4483, t14364, t2907, t4496);
        let (t14482, t14484, t14486, t14487) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1571::<F>(t14480, t959, t2952, t4483, t10623, t1589, t14257, t14262, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14424, t14472, t14475, t14477, t14479);
    (t14391, t14394, t14398, t14424, t14472, t14475, t14477, t14479, t14482, t14484, t14486, t14487)
}
