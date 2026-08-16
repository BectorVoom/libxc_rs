//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 913/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk913(t22573: f64, t6875: f64, t111: f64, t22558: f64, t7002: f64, t112: f64, t23862: f64, t7222: f64, t24447: f64, t24007: f64, t225: f64, t24141: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t83886 = t6875 * t22573;
    let t83935 = t22558 * t111;
    let t83980 = t7002 * t111;
    let t84004 = t23862 * t112;
    let t84033 = t7222 * t111;
    let t84078 = t24447 * t112;
    let t84097 = t24007 * t111;
    let t84433 = t24141 * t225;
    (t83886, t83935, t83980, t84004, t84033, t84078, t84097, t84433)
}
