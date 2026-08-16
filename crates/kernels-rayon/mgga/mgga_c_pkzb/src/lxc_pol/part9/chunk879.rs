//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 879/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk879(t154: f64, t6106: f64, t907: f64, t405: f64, t486: f64, t824: f64, t385: f64, t2185: f64, t2347: f64, t5717: f64, t913: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6443 = t154 * t907 * t6106;
    let t6446 = t486 * t405;
    let t6448 = t154 * t6446 * t824;
    let t6449 = t385 * t6448;
    let t6452 = t154 * t2347 * t2185;
    let t6453 = t385 * t6452;
    let t6455 = t5717 * t913;
    (t6443, t6446, t6448, t6449, t6452, t6453, t6455)
}
