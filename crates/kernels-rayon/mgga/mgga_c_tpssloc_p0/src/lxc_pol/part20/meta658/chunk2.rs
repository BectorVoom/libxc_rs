//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2442/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2442(t10402: f64, t14618: f64, t14608: f64, t13969: f64, t13981: f64, t3130: f64, t10422: f64, t14129: f64, t3070: f64, t11002: f64, t14508: f64, t10895: f64, t14511: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49929 = t14618 * t10402;
    let t49934 = t14608 * t10402;
    let t49940 = t3130 * t13969 * t13981;
    let t49945 = t3070 * t10422 * t14129;
    let t49957 = t14508 * t11002;
    let t49959 = t14511 * t10895;
    (t49929, t49934, t49940, t49945, t49957, t49959)
}
