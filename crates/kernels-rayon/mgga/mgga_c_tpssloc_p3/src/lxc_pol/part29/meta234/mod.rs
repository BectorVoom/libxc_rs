//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta234 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1093;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1094;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1095;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1096;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1097;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1098;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta234(t1340: f64, t5234: f64, t1358: f64, t1815: f64, t1362: f64, t242: f64, t3788: f64, t1336: f64, t557: f64, t67: f64, t246: f64, t120: f64, t1824: f64, t1351: f64, t3792: f64, t1827: f64, t3799: f64, t1315: f64, t1354: f64, t1369: f64, t3733: f64, t3762: f64, t3763: f64, t3778: f64, t5220: f64, t5223: f64, t5227: f64, t5231: f64, t559: f64, t3805: f64, t3807: f64, t2408: f64, t2417: f64, t2423: f64, t3686: f64, t3688: f64, t3690: f64, t3695: f64, t3813: f64, t5153: f64, t5156: f64, t5159: f64, t5164: f64, t5167: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5235, t5238, t5240, t5245, t5246, t5247, t5248) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1093(t1340, t5234, t1358, t1815, t1362, t242, t3788, t1336, t557, t67, t246);
        let t5249 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1094(t120, t1824);
        let t5250 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1095(t1351, t3792);
        let t5252 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1096(t5248, t5249, t5250);
        let t5257 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1097(t1827, t3799, t1315, t1354, t1369, t3733, t3762, t3763, t3778, t5220, t5223, t5227, t5231, t5235, t5238, t5240, t5246, t5252, t559);
        let t5259 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1098(t3805, t3807, t5249);
        let t5262 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1099(t2408, t2417, t2423, t3686, t3688, t3690, t3695, t3813, t5153, t5156, t5159, t5164, t5167);
    (t5235, t5240, t5245, t5246, t5247, t5248, t5249, t5250, t5252, t5257, t5259, t5262)
}
