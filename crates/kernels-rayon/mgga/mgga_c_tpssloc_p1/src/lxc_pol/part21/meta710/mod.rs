//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta710 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2544;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2545;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta710(t10403: f64, t10422: f64, t14214: f64, t3030: f64, t4552: f64, t3032: f64, t3129: f64, t13998: f64, t2960: f64, t42875: f64, t4338: f64, t973: f64, t14040: f64, t3070: f64, t10516: f64, t4640: f64, t14121: f64, t13748: f64, t13965: f64, t3114: f64, t14202: f64, t3117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49629, t49649, t49650, t49651, t49658, t49661) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2544(t10403, t10422, t14214, t3030, t4552, t3032, t3129, t13998, t2960, t42875, t4338, t973);
        let (t49666, t49678, t49682, t49684, t49690, t49692) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2545(t10422, t14040, t3070, t10516, t4640, t10403, t14121, t13748, t2960, t13965, t3114, t14202, t3117);
    (t49629, t49649, t49650, t49651, t49658, t49661, t49666, t49678, t49682, t49684, t49690, t49692)
}
