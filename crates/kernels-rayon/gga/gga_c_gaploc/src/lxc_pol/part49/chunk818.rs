//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 818/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk818(t2021: f64, t8774: f64, t10007: f64, t8669: f64, t2925: f64, t5750: f64, t10555: f64, t161: f64, t197: f64, t2754: f64, t1: f64, t20550: f64, t7892: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25198 = t2021 * t8774;
    let t25359 = t10007 * t8669;
    let t25405 = t5750 * t2925;
    let t25718 = t10555 * t161;
    let t25760 = t197 * t2754;
    let t26126 = t25760 * t1;
    let t26328 = t20550 * t7892;
    (t25198, t25359, t25405, t25718, t25760, t26126, t26328)
}
