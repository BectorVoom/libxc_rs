//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 648/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk648(t10915: f64, t240: f64, t2917: f64, t342: f64, t4910: f64, t630: f64, t1882: f64, t4923: f64, t4917: f64, t9570: f64, t9577: f64, t226: f64, t2383: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17687 = t10915 * t240;
    let t17694 = t2917 * t240;
    let t17703 = t342 * t630 * t4910;
    let t17720 = t1882 * t4923;
    let t17748 = t9570 * t4917;
    let t17765 = t9577 * t4917;
    let t17818 = t2383 * t226;
    (t17687, t17694, t17703, t17720, t17748, t17765, t17818)
}
