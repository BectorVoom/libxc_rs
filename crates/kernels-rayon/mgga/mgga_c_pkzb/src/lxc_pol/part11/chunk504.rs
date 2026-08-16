//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 504/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk504(t1025: f64, t1702: f64, t1024: f64, t568: f64, t581: f64, t2575: f64, t50: f64, t168: f64, t1717: f64) -> (f64, f64, f64, f64, f64) {
    let t2580 = t1702 * t1025;
    let t2583 = t581 * t1024 * t568;
    let t2586 = t50 * t2575;
    let t2587 = t581 * t2586;
    let t2590 = t1717 * t168;
    (t2580, t2583, t2586, t2587, t2590)
}
