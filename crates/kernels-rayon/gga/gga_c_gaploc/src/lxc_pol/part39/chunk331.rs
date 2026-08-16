//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 331/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk331(t2628: f64, t958: f64, t1457: f64, t2582: f64, t2571: f64, t723: f64, t1445: f64, t2541: f64, t313: f64, t1645: f64, t740: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2629 = t958 * t2628;
    let t2631 = t1457 * t2582;
    let t2634 = t2571 * t723;
    let t2635 = t1445 * t2634;
    let t2638 = t313 * t2541;
    let t2639 = t1645 * t740;
    (t2629, t2631, t2634, t2635, t2638, t2639)
}
