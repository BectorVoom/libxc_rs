//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1061/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1061(t41305: f64, t41307: f64, t43841: f64, t43849: f64, t43858: f64, t43861: f64, t43864: f64, t43875: f64, t43879: f64, t43881: f64, t43883: f64, t43884: f64, t43885: f64, t43886: f64, t43887: f64, t43888: f64, t43889: f64, t43892: f64, t43893: f64, t47408: f64) -> f64 {
    let t51146 = t43841 - t43849 - t43858 + t43861 + t43864 + t43875 - t43879 + 0.76685851907841499352e0_f64 * t43881 + t43883 + t43884 - t43885 - t43886 + t43887 - t43888 - t43889 + 0.59584149919750711115e-1_f64 * t41305 - 0.89376224879626066675e-1_f64 * t41307 + t43892 + t43893 - t47408;
    t51146
}
