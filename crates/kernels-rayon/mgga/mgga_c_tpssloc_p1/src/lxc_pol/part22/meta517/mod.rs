//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1982;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1983;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta517(t21812: f64, t21815: f64, t21829: f64, t21832: f64, t21835: f64, t21956: f64, t21958: f64, t21960: f64, t21963: f64, t22224: f64, t22226: f64, t11292: f64, t21906: f64, t3403: f64, t1164: f64, t1147: f64, t1156: f64, t21938: f64, t11282: f64, t11285: f64, t4869: f64, t6102: f64, t21726: f64, t21728: f64, t21730: f64, t21732: f64, t21897: f64, t21901: f64, t21990: f64, t21993: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22227, t22228) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1982(t21812, t21815, t21829, t21832, t21835, t21956, t21958, t21960, t21963, t22224, t22226, t11292, t21906);
        let (t22229, t22231, t22233, t22235, t22236, t22237, t22239, t22241, t22242) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1983(t22228, t3403, t1164, t1147, t1156, t21938, t11282, t21906, t11285, t4869, t6102, t21726, t21728, t21730, t21732, t21897, t21901, t21990, t21993);
    (t22227, t22228, t22229, t22231, t22233, t22235, t22236, t22237, t22239, t22241, t22242)
}
