//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 484/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk484(t192: f64, t8071: f64, t1564: f64, t2754: f64, t2765: f64, t524: f64, t188: f64, t7930: f64, t493: f64, t7892: f64, t1397: f64, t2897: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8072 = t8071 * t192;
    let t8097 = t1564 * t2754;
    let t8155 = t524 * t2765;
    let t8158 = t188 * t7930;
    let t8195 = t493 * t7892;
    let t8229 = t1397 * t2897;
    (t8072, t8097, t8155, t8158, t8195, t8229)
}
