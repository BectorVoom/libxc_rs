//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1970;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1971;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1972;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta629(t1877: f64, t2057: f64, t584: f64, t9212: f64, t2219: f64, t7110: f64, t26756: f64, t86732: f64, t86843: f64, t86868: f64, t86870: f64, t225: f64, t26722: f64, t86886: f64, t86895: f64, t2053: f64, t40889: f64, t10049: f64, t13049: f64, t25168: f64, t26713: f64, t2743: f64, t7842: f64, t866: f64, t86847: f64, t86852: f64, t86857: f64, t86862: f64, t86866: f64, t86875: f64, t86881: f64, t86884: f64, t86891: f64, t86901: f64, t86903: f64, t86911: f64, t86916: f64, t86928: f64, t86940: f64, t86942: f64, t13029: f64, t2047: f64, t259: f64, t26700: f64, t4142: f64, t7084: f64, t82079: f64, t82082: f64, t82087: f64, t86933: f64, t9590: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92356, t92359, t92362, t92364, t92375, t92382, t92383, t92386) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1970(t1877, t2057, t584, t9212, t2219, t7110, t26756, t86732, t86843, t86868, t86870, t225, t26722);
        let t92400 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1971(t86886, t86895, t2053, t40889, t10049, t13049, t25168, t26713, t2743, t7842, t866, t86847, t86852, t86857, t86862, t86866, t86875, t86881, t86884, t86891, t86901, t86903, t92375, t92382, t92383, t92386);
        let (t92402, t92406, t92428) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1972(t86911, t86916, t86928, t86940, t86942, t13029, t2047, t259, t26700, t2743, t4142, t7084, t7842, t82079, t82082, t82087, t86933, t9590);
    (t92356, t92359, t92362, t92364, t92400, t92402, t92406, t92428)
}
