//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta746 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2481;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2482;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2483;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2484;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2485;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2486;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta746(t18030: f64, t4630: f64, t17884: f64, t4644: f64, t13969: f64, t21502: f64, t3039: f64, t10214: f64, t1041: f64, t14080: f64, t14164: f64, t21603: f64, t2979: f64, t3048: f64, t4582: f64, t47775: f64, t5861: f64, t62282: f64, t62284: f64, t68521: f64, t68534: f64, t68539: f64, t70330: f64, t70339: f64, t973: f64, t977: f64, t1023: f64, t14218: f64, t14508: f64, t17673: f64, t17701: f64, t17734: f64, t21138: f64, t21597: f64, t3070: f64, t3071: f64, t3114: f64, t42388: f64, t42752: f64, t4650: f64, t48570: f64, t48611: f64, t49853: f64, t49872: f64, t49934: f64, t5681: f64, t62306: f64, t69935: f64, t21550: f64, t10937: f64, t17697: f64, t21570: f64, t2986: f64, t42358: f64, t43361: f64, t49907: f64, t49923: f64, t50366: f64, t62343: f64, t62349: f64, t62360: f64, t62840: f64, t68513: f64, t70273: f64, t135: f64, t21537: f64, t21541: f64, t21545: f64, t13995: f64, t18041: f64, t10390: f64, t48496: f64, t49984: f64, t5909: f64, t62418: f64, t68458: f64, t68466: f64, t68470: f64, t68543: f64, t68547: f64, t68554: f64, t17659: f64, t10422: f64, t21573: f64, t10408: f64, t21516: f64, t21520: f64, t21574: f64, t3117: f64, t4337: f64, t49994: f64, t50048: f64, t5857: f64, t62441: f64, t62445: f64, t70442: f64, t25548: f64, t360: f64, t10403: f64, t17177: f64, t17182: f64, t17920: f64, t17925: f64, t17972: f64, t3130: f64, t4594: f64, t62494: f64, t62499: f64, t62510: f64, t62515: f64, t70082: f64, t70391: f64, t1036: f64, t21483: f64, t21511: f64, t10413: f64, t21531: f64, t14511: f64, t17718: f64, t18021: f64, t21396: f64, t21595: f64, t48607: f64, t50148: f64, t50170: f64, t62602: f64, t69657: f64, t884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70554, t70599) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2481(t18030, t4630, t17884, t4644, t13969, t21502, t3039, t10214, t1041, t14080, t14164, t21603, t2979, t3048, t4582, t47775, t5861, t62282, t62284, t68521, t68534, t68539, t70330, t70339, t973, t977);
        let t70623 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2482(t1023, t14218, t14508, t17673, t17701, t17734, t21138, t21597, t3070, t3071, t3114, t42388, t42752, t4650, t48570, t48611, t49853, t49872, t49934, t5681, t62306, t69935);
        let t70645 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2483(t1041, t13969, t21550, t1023, t10937, t14218, t17697, t21570, t2986, t42358, t43361, t4582, t4644, t48611, t49907, t49923, t50366, t62343, t62349, t62360, t62840, t68513, t70273);
        let (t70655, t70660, t70665, t70707) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2484(t135, t21537, t973, t21541, t21545, t13995, t18041, t10390, t1041, t21570, t2979, t4582, t48496, t49984, t5909, t62418, t68458, t68466, t68470, t68543, t68547, t68554, t70330, t977);
        let t70728 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2485(t17659, t4644, t10422, t21573, t3070, t10390, t10408, t10937, t14080, t21516, t21520, t21574, t3117, t4337, t49994, t50048, t5857, t62441, t62445, t70442);
        let t70756 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2486(t25548, t360, t10403, t10408, t13995, t17177, t17182, t17920, t17925, t17972, t3070, t3071, t3130, t4582, t4594, t4644, t49934, t5681, t62494, t62499, t62510, t62515, t70082, t70391);
        let (t70766, t70802) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2487(t1036, t21483, t1041, t13969, t21511, t10413, t10422, t21531, t10408, t10937, t13995, t14511, t17718, t18021, t21396, t21520, t21595, t3070, t3071, t43361, t48607, t50148, t50170, t62602, t69657, t884);
    (t70554, t70599, t70623, t70645, t70655, t70660, t70665, t70707, t70728, t70756, t70766, t70802)
}
