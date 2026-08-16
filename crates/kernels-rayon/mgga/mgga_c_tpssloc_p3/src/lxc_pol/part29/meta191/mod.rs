//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta191 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk992;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk993;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta191(t2627: f64, t68: f64, t226: f64, t1509: f64, t252: f64, t4182: f64, t1510: f64, t2732: f64, t4234: f64, t860: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4280, t4281) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk992(t2627, t68, t226);
        let t4282 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk993(t1509, t252);
        let (t4283, t4286, t4288, t4290, t4291) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk994(t4182, t4282, t1510, t2732, t4234, t860, t68, t814, t226);
    (t4280, t4281, t4282, t4283, t4286, t4288, t4290, t4291)
}
