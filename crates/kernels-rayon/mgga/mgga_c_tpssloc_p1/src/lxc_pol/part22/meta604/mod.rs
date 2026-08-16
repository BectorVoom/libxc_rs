//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2126;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2127;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta604(t10828: f64, t300: f64, t2930: f64, t3030: f64, t4552: f64, t3032: f64, t3129: f64, t42875: f64, t4338: f64, t973: f64, t13965: f64, t3114: f64, t14202: f64, t3117: f64, t10890: f64, t14507: f64, t3038: f64, t1020: f64, t10508: f64, t248: f64, t4650: f64, t3109: f64, t247: f64, t677: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49532, t49541, t49649, t49650, t49651, t49662, t49690) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2126(t10828, t300, t2930, t3030, t4552, t3032, t3129, t42875, t4338, t973, t13965, t3114);
        let (t49691, t49693, t49743, t49771, t49819, t49831) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2127(t49690, t14202, t3117, t10890, t14507, t3038, t49650, t1020, t10508, t248, t4650, t13965, t3109);
        let (t49832, t49850) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2128(t49831, t247, t677);
    (t49532, t49541, t49649, t49651, t49662, t49691, t49693, t49743, t49771, t49819, t49832, t49850)
}
