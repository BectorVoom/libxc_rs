//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1050/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1050(t2133: f64, t3916: f64, t2138: f64, t3111: f64, t3763: f64, t2255: f64, t1109: f64, t745: f64, t3258: f64, t3717: f64, t5: f64, t337: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11794 = t3916 * t2133;
    let t11796 = t11794 * t2138 / 96.0_f64;
    let t11797 = t3111 * t3763;
    let t11798 = t2255 * t11797;
    let t11801 = t745 * t1109;
    let t11803 = t2255 * t3258 * t11801;
    let t11806 = t5 * t3717;
    let t11807 = t337 * t11806;
    (t11794, t11796, t11797, t11798, t11803, t11806, t11807)
}
