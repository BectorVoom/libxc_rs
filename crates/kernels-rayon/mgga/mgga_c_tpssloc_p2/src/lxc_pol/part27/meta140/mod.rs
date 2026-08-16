//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta140 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk796;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk797;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk798;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta140(t3076: f64, t68: f64, t369: f64, t374: f64, t376: f64, t677: f64, t370: f64, t35: f64, t365: f64, t612: f64, t364: f64, t354: f64, t1032: f64, t1036: f64, t1004: f64, t1031: f64, t1044: f64, t248: f64, t2776: f64, t121: f64, t1023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3077, t3078, t3082) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk796(t3076, t68, t369, t374, t376, t677);
        let (t3084, t3087, t3088, t3089, t3092, t3094, t3098) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk797(t3082, t370, t35, t365, t612, t364, t354, t1032, t1036, t1004, t1031, t1044, t248, t2776);
        let t3101 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk798(t121, t376);
        let t3103 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk799(t1023, t248, t3101);
    (t3077, t3078, t3082, t3084, t3087, t3088, t3089, t3092, t3094, t3098, t3101, t3103)
}
