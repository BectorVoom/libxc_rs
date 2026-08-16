//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2104/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2104(t91548: f64, t2022: f64, t6483: f64, t671: f64, t28821: f64, t6997: f64, t1441: f64, t4072: f64, t1874: f64, t28002: f64, t6525: f64, t7450: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93906 = 0.3289868133696452873e-1_f64 * t91548;
    let t96348 = t2022 * t6483;
    let t96351 = t2022 * t671;
    let t96355 = t28821 * t6997;
    let t96356 = t1441 * t4072;
    let t96358 = 4.0_f64 * t96356 * t1874;
    let t96360 = 4.0_f64 * t28002 * t6525;
    let t96361 = t7450 * t671;
    (t93906, t96348, t96351, t96355, t96356, t96358, t96360, t96361)
}
