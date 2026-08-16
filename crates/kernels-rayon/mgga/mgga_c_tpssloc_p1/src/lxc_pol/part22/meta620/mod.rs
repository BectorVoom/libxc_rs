//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2151;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta620(t1214: f64, t820: f64, t3624: f64, t52627: f64, t43763: f64, t44827: f64, t3515: f64, t4983: f64, t49850: f64, t11818: f64, t1213: f64, t248: f64, t5012: f64, t11820: f64, t5019: f64, t11791: f64, t5024: f64, t5002: f64, t11153: f64, t4899: f64, t3540: f64, t4961: f64, t1227: f64, t4973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52897, t52903, t52919, t52953, t52973) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2151(t1214, t820, t3624, t52627, t43763, t44827, t3515, t4983, t49850, t11818, t1213, t248, t5012);
        let (t52974, t52988, t52992, t52994, t52995, t53000, t53033) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2152(t52973, t11820, t5019, t11791, t5024, t5002, t11153, t4899, t3540, t4961, t1227, t4973, t49850);
    (t52897, t52903, t52919, t52953, t52974, t52988, t52992, t52994, t52995, t53000, t53033)
}
