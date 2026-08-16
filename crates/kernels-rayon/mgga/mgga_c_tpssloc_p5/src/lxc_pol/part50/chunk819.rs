//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 819/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk819(t345: f64, t8400: f64, t383: f64, t8391: f64, t1920: f64, t353: f64, t1055: f64) -> (f64, f64, f64, f64) {
    let t8401 = t345 * t8400;
    let t8404 = t383 * t8391;
    let t8406 = 0.16449340668482264365e-1_f64 * t1920 * t8401 + t353 * t8404;
    let t8407 = t1055 * t8406;
    (t8401, t8404, t8406, t8407)
}
