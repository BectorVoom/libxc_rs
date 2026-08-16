//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1251/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1251(t11181: f64, t11262: f64, t4865: f64, t11236: f64, t4015: f64, t11235: f64, t14940: f64, t1603: f64, t8286: f64, t11242: f64, t203: f64, t2922: f64, t8296: f64) -> (f64, f64, f64, f64) {
    let t35432 = t11181 * t4865 * t11262;
    let t35435 = t11181 * t4015 * t11236;
    let t35439 = t8286 * t14940 * t11235 * t1603;
    let t35443 = t2922 * t11242 * t203 * t8296;
    (t35432, t35435, t35439, t35443)
}
