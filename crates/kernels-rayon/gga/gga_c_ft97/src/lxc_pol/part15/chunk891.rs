//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 891/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk891(t309: f64, t44600: f64, t294: f64, t9577: f64, t9570: f64, t342: f64, t784: f64, t8639: f64, t43537: f64, t3051: f64, t963: f64, t926: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44601 = t309 * t44600;
    let t44674 = t294 * t9577;
    let t44700 = t294 * t9570;
    let t44716 = 5.0_f64 / 54.0_f64 * t342 * t8639 * t784;
    let t44776 = 140.0_f64 / 243.0_f64 * t43537;
    let t44950 = t3051 * t963;
    let t45304 = t3051 * t926;
    (t44601, t44674, t44700, t44716, t44776, t44950, t45304)
}
