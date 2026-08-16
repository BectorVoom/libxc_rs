//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 781/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk781(t2365: f64, t31586: f64, t4391: f64, t31591: f64, t12960: f64, t31051: f64, t10473: f64, t2478: f64, t6576: f64, t34688: f64, t9272: f64, t9273: f64) -> (f64, f64, f64, f64, f64) {
    let t41626 = t4391 * t2365 * t31586;
    let t41629 = t4391 * t2365 * t31591;
    let t41645 = t31051 * t12960;
    let t41649 = t6576 * t10473 * t2478;
    let t41656 = t9272 * t34688 * t9273;
    (t41626, t41629, t41645, t41649, t41656)
}
