//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2102/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2102(t39063: f64, t6489: f64, t22573: f64, t6875: f64, t22947: f64, t532: f64, t111: f64, t22558: f64, t7002: f64, t112: f64, t23862: f64, t1395: f64, t7020: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83830 = t39063 * t6489;
    let t83886 = t6875 * t22573;
    let t83929 = t532 * t22947;
    let t83935 = t22558 * t111;
    let t83980 = t7002 * t111;
    let t84004 = t23862 * t112;
    let t84024 = t1395 * t7020;
    (t83830, t83886, t83929, t83935, t83980, t84004, t84024)
}
