//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1460;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1461;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta298(t3117: f64, t4571: f64, t248: f64, t3051: f64, t4347: f64, t1041: f64, t3114: f64, t4630: f64, t3101: f64, t4650: f64, t1020: f64, t10508: f64, t1616: f64, t122: f64, t247: f64, t4599: f64, t3039: f64, t3069: f64, t4669: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13948, t13950, t13952, t13959, t13961, t13963, t13965) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1460(t3117, t4571, t248, t3051, t4347, t1041, t3114, t4630, t3101, t4650, t1020, t10508, t1616);
        let (t13966, t13969) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1461(t1020, t13965, t122, t247);
        let (t13970, t13972, t13995) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1462(t13969, t4599, t3039, t3069, t4669);
    (t13948, t13950, t13952, t13959, t13961, t13963, t13965, t13966, t13969, t13970, t13972, t13995)
}
