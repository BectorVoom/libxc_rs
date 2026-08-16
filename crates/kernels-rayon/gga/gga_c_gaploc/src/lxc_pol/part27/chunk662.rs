//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 662/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk662(t4416: f64, t823: f64, t1422: f64, t2031: f64, t2021: f64, t2200: f64, t832: f64, t19: f64, t2084: f64) -> (f64, f64, f64, f64) {
    let t5586 = t823 * t4416;
    let t5597 = t2031 * t1422;
    let t5598 = t2021 * t5597;
    let t5629 = t2200 * t832;
    let t5638 = t2084 * t19;
    (t5586, t5598, t5629, t5638)
}
