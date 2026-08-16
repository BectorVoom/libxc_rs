//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2171;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta520(t2970: f64, t5828: f64, t973: f64, t16558: f64, t978: f64, t977: f64, t343: f64, t5836: f64, t984: f64, t4546: f64, t10231: f64, t5817: f64, t13861: f64, t4531: f64, t17178: f64, t4510: f64, t2989: f64, t5398: f64, t2988: f64, t10186: f64, t13830: f64, t13850: f64, t2960: f64, t2986: f64, t5818: f64, t5821: f64, t5829: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17769, t17770, t17772, t17773, t17777, t17778, t17783) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2171(t2970, t5828, t973, t16558, t978, t977, t343, t5836, t984, t4546, t10231, t5817);
        let (t17788, t17791, t17794, t17795, t17798) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2172(t17783, t973, t13861, t4531, t17178, t4510, t2989, t5398, t2988, t10186, t13830, t13850, t17770, t17773, t17778, t2960, t2986, t5818, t5821, t5829);
    (t17769, t17772, t17773, t17777, t17778, t17783, t17788, t17791, t17794, t17795, t17798)
}
