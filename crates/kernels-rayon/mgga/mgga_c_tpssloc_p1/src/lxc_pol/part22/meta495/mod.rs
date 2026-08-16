//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1922;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta495(t10214: f64, t21468: f64, t20234: f64, t2980: f64, t977: f64, t21126: f64, t4518: f64, t13909: f64, t17784: f64, t17809: f64, t21430: f64, t21433: f64, t21447: f64, t21453: f64, t21459: f64, t21463: f64, t2986: f64, t973: f64, t21429: f64, t225: f64, t68: f64, t369: f64, t14211: f64, t17712: f64, t4582: f64, t21122: f64, t2979: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21469, t21472, t21473, t21476, t21479) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1922(t10214, t21468, t20234, t2980, t977, t21126, t4518, t13909, t17784, t17809, t21430, t21433, t21447, t21453, t21459, t21463, t2986, t973);
        let (t21480, t21481, t21482, t21483, t21486, t21487, t21490, t21493) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1923(t21429, t21479, t225, t68, t369, t14211, t17712, t4582, t21126, t977, t21122, t2979);
    (t21469, t21472, t21473, t21476, t21480, t21481, t21482, t21483, t21486, t21487, t21490, t21493)
}
