//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta381 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1565;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1566;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1567;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1568;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1569;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1570;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1571;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta381(t2844: f64, t4395: f64, t912: f64, t2842: f64, t2836: f64, t4399: f64, t10704: f64, t1556: f64, t2793: f64, t10702: f64, t13566: f64, t13602: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10832: f64, t13563: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13598: f64, t13613: f64, t1557: f64, t4434: f64, t931: f64, t10740: f64, t10765: f64, t14376: f64, t14378: f64, t14381: f64, t14384: f64, t14387: f64, t2861: f64, t311: f64, t4416: f64, t4438: f64, t1569: f64, t2880: f64, t2862: f64, t4437: f64, t2888: f64, t4433: f64, t10813: f64, t1568: f64, t4472: f64, t950: f64, t1581: f64, t2924: f64, t2906: f64, t4475: f64, t2932: f64, t4471: f64, t10747: f64, t10771: f64, t10811: f64, t10825: f64, t10828: f64, t2886: f64, t2905: f64, t2930: f64, t4454: f64, t4476: f64, t14279: f64, t14373: f64, t300: f64, t4446: f64, t961: f64, t2948: f64, t4483: f64, t14364: f64, t2907: f64, t4496: f64, t959: f64, t2952: f64, t10623: f64, t1589: f64, t14257: f64, t14262: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14391, t14394, t14398, t14409, t14410) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1565(t2844, t4395, t912, t2842, t2836, t4399, t10704, t1556, t2793, t10702, t13566, t13602);
        let t14419 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1566(t10556, t10558, t10560, t10562, t10832, t13563, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613, t14409, t14410);
        let (t14424, t14428) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1567(t1557, t2793, t2842, t4434, t931, t10740, t10765, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14419, t2861, t311, t4416, t4438);
        let (t14429, t14432, t14436, t14439, t14443, t14450, t14453) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1568(t1569, t2880, t2862, t4437, t2888, t4433, t931, t10813, t1568, t4472, t950, t1581, t2924);
        let t14469 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1569(t2906, t4475, t2932, t4471, t950, t1581, t1569, t2862, t10747, t10771, t10811, t10825, t10828, t14429, t14432, t14436, t14439, t14443, t14450, t14453, t2861, t2886, t2905, t2930, t4454, t4476);
        let (t14472, t14475, t14477, t14479, t14480) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1570(t14279, t14373, t14428, t14469, t300, t4446, t961, t2948, t4483, t14364, t2907, t4496);
        let (t14482, t14484, t14486, t14487) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1571(t14480, t959, t2952, t4483, t10623, t1589, t14257, t14262, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14424, t14472, t14475, t14477, t14479);
    (t14391, t14394, t14398, t14424, t14472, t14475, t14477, t14479, t14482, t14484, t14486, t14487)
}
