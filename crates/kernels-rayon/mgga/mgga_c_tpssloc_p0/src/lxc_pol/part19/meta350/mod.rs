//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1272;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1273;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1274;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1275;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1276;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta350(t2904: f64, t41733: f64, t951: f64, t959: f64, t41654: f64, t41642: f64, t41646: f64, t41651: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41669: f64, t41673: f64, t41675: f64, t41678: f64, t41680: f64, t41682: f64, t41684: f64, t41690: f64, t41695: f64, t41699: f64, t41703: f64, t41707: f64, t41711: f64, t41713: f64, t41717: f64, t324: f64, t10603: f64, t2932: f64, t10717: f64, t10720: f64, t10724: f64, t10734: f64, t10740: f64, t10747: f64, t10753: f64, t10756: f64, t10765: f64, t10771: f64, t10825: f64, t10828: f64, t14259: f64, t2880: f64, t2889: f64, t2905: f64, t2924: f64, t2930: f64, t2933: f64, t41620: f64, t41622: f64, t41625: f64, t41627: f64, t41635: f64, t41639: f64, t41722: f64, t950: f64, t2794: f64, t2836: f64, t2842: f64, t2784: f64, t2791: f64, t2897: f64, t2929: f64, t10629: f64, t938: f64, t2903: f64, t2928: f64, t315: f64, t2906: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41737, t41749) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1272(t2904, t41733, t951, t959, t41654, t41642, t41646, t41651, t41656, t41658, t41660, t41662, t41669, t41673, t41675);
        let t41762 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1273(t41678, t41680, t41682, t41684, t41690, t41695, t41699, t41703, t41707, t41711, t41713, t41717);
        let (t41764, t41790) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1274(t324, t41749, t41762, t10603, t2932, t10717, t10720, t10724, t10734, t10740, t10747, t10753, t10756, t10765, t10771, t10825, t10828, t14259, t2880, t2889, t2905, t2924, t2930, t2933, t41620, t41622, t41625, t41627, t41635, t41639, t41722, t950);
        let (t41804, t41813, t41816, t41821, t41825, t41826) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1275(t2794, t2836, t2842, t2784, t2791, t2897, t2929, t10629, t938, t2903, t2928, t315);
        let t41827 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1276(t2906);
    (t41737, t41764, t41790, t41804, t41813, t41816, t41821, t41825, t41826, t41827)
}
