//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1112/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1112(t143373: f64, t2665: f64, t446: f64, t992: f64, t35854: f64, t6308: f64, t681: f64, t33288: f64, t35834: f64, t7638: f64, t35829: f64, t33811: f64, t35820: f64) -> (f64, f64, f64, f64, f64) {
    let t152931 = t446 * t2665 * t143373 * t992;
    let t152934 = t6308 * t681 * t35854;
    let t152937 = t7638 * t33288 * t35834;
    let t152940 = t7638 * t33288 * t35829;
    let t152943 = t33811 * t33288 * t35820;
    (t152931, t152934, t152937, t152940, t152943)
}
