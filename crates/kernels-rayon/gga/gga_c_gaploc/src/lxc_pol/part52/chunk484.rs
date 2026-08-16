//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 484/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk484(t1: f64, t8773: f64, t2021: f64, t1890: f64, t2925: f64, t1033: f64, t1959: f64, t161: f64, t1023: f64, t1853: f64, t1022: f64, t2101: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8792 = t8773 * t1;
    let t8793 = t2021 * t8792;
    let t8802 = t1890 * t2925;
    let t8862 = t1033 * t1959;
    let t8878 = t8773 * t161;
    let t8942 = t1023 * t1853;
    let t9014 = t2101 * t1022;
    (t8792, t8793, t8802, t8862, t8878, t8942, t9014)
}
