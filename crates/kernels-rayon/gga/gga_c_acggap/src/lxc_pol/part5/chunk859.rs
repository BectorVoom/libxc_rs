//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 859/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk859(t864: f64, t879: f64, t317: f64, t3922: f64, t3915: f64, t3937: f64, t3889: f64, t852: f64, t3919: f64, t3868: f64, t1264: f64, t449: f64, t863: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12235 = t864 * t879;
    let t12238 = 0.23707617058567841754e2_f64 * t3922 * t317 * t12235;
    let t12240 = 0.15805078039045227836e2_f64 * t3937 * t3915;
    let t12241 = t852 * t3889;
    let t12243 = t3937 * t3919;
    let t12246 = t3868 * t3919;
    let t12250 = t863 * t449 * t864 * t1264;
    (t12235, t12238, t12240, t12241, t12243, t12246, t12250)
}
