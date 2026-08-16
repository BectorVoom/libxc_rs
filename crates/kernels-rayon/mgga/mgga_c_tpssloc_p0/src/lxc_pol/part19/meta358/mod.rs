//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1299;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1300;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1301;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1302;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1303;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta358(t2884: f64, t302: f64, t2887: f64, t10727: f64, t10817: f64, t10655: f64, t10731: f64, t10661: f64, t2836: f64, t2845: f64, t10697: f64, t2792: f64, t912: f64, t41654: f64, t41642: f64, t41646: f64, t41651: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41669: f64, t41673: f64, t41675: f64, t41678: f64, t41680: f64, t41682: f64, t41684: f64, t41690: f64, t41695: f64, t41699: f64, t41703: f64, t41707: f64, t41711: f64, t41713: f64, t41717: f64, t10756: f64, t10806: f64, t10813: f64, t10814: f64, t10828: f64, t10829: f64, t2856: f64, t2889: f64, t2905: f64, t2930: f64, t2932: f64, t311: f64, t41733: f64, t41827: f64, t41987: f64, t42123: f64, t42128: f64, t42145: f64, t42148: f64, t42149: f64, t42154: f64, t42172: f64, t42187: f64, t42203: f64, t42218: f64, t924: f64, t932: f64, t951: f64, t300: f64, t41790: f64, t41993: f64, t42122: f64, t1068: f64, t11087: f64, t3216: f64, t41620: f64, t41622: f64, t41625: f64, t41627: f64, t41635: f64, t41639: f64, t41722: f64, t41726: f64, t41728: f64, t41732: f64, t41737: f64, t4700: f64, t10633: f64, t2940: f64, t10629: f64, t959: f64, t10619: f64, t961: f64, t10957: f64, t3053: f64, t271: f64, t2770: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42226, t42228, t42233, t42235, t42238, t42241) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1299(t2884, t302, t2887, t10727, t10817, t10655, t10731, t10661, t2836, t2845, t10697, t2792, t912);
        let (t42253, t42266) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1300(t41654, t41642, t41646, t41651, t41656, t41658, t41660, t41662, t41669, t41673, t41675, t41678, t41680, t41682, t41684, t41690, t41695, t41699, t41703, t41707, t41711, t41713, t41717);
        let t42270 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1301(t10756, t10806, t10813, t10814, t10828, t10829, t2856, t2889, t2905, t2930, t2932, t311, t41733, t41827, t41987, t42123, t42128, t42145, t42148, t42149, t42154, t42172, t42187, t42203, t42218, t42226, t42228, t42233, t42235, t42238, t42241, t42253, t42266, t924, t932, t951);
        let (t42273, t42274) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1302(t300, t41790, t41993, t42122, t42270, t1068, t11087, t3216, t41620, t41622, t41625, t41627, t41635, t41639, t41722, t41726, t41728, t41732, t41737, t4700);
        let (t42276, t42280, t42283, t42303, t42308) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1303(t10633, t2940, t10629, t2932, t41827, t959, t10619, t300, t961, t10957, t3053, t271, t2770);
    (t42233, t42235, t42238, t42241, t42273, t42274, t42276, t42280, t42283, t42303, t42308)
}
