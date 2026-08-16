//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta711 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2546;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2547;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta711(t10423: f64, t13995: f64, t10413: f64, t10422: f64, t14221: f64, t10949: f64, t14025: f64, t10883: f64, t13969: f64, t14106: f64, t13559: f64, t2970: f64, t973: f64, t1036: f64, t13942: f64, t3047: f64, t4616: f64, t10890: f64, t14507: f64, t1041: f64, t14188: f64, t1020: f64, t14489: f64, t248: f64, t3101: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49697, t49702, t49716, t49721, t49732) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2546(t10423, t13995, t10413, t10422, t14221, t10949, t14025, t10883, t13969, t14106, t13559, t2970, t973);
        let (t49734, t49740, t49743, t49748, t49757) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2547(t1036, t13942, t3047, t4616, t10890, t14507, t1041, t13969, t14188, t1020, t14489, t248, t3101);
    (t49697, t49702, t49716, t49721, t49732, t49734, t49740, t49743, t49748, t49757)
}
