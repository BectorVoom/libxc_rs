//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1011/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1011(t43586: f64, t7572: f64, t7573: f64, t10811: f64, t9961: f64, t1022: f64, t9636: f64, t2009: f64, t2021: f64, t13150: f64, t2013: f64, t10007: f64, t2925: f64, t825: f64, t9438: f64) -> (f64, f64, f64, f64, f64) {
    let t44076 = t7572 * t7573 * t43586;
    let t44079 = 0.85801175884441024006e1_f64 * t10811 * t9961;
    let t44080 = t9636 * t1022;
    let t44083 = 0.35750489951850426669e0_f64 * t2021 * t44080 * t2009;
    let t44084 = t2013 * t13150;
    let t44085 = 0.15976219147466979032e-1_f64 * t44084;
    let t44088 = t825 * t9438 * t10007 * t2925;
    (t44076, t44079, t44083, t44085, t44088)
}
