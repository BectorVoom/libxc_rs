//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 757/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk757(t10248: f64, t10249: f64, t446: f64, t2409: f64, t824: f64, t2665: f64, t792: f64, t8608: f64, t666: f64, t89: f64, t191: f64, t7640: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10250 = t10248 * t10249;
    let t10251 = t446 * t10250;
    let t10253 = t2409 * t824;
    let t10254 = t2665 * t10253;
    let t10255 = t446 * t10254;
    let t10257 = t792 * t8608;
    let t10259 = t89 * t666 * t10257;
    let t10261 = t191 * t7640;
    (t10250, t10251, t10253, t10254, t10255, t10257, t10259, t10261)
}
