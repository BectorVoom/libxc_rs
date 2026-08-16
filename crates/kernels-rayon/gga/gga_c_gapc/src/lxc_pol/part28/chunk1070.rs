//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1070/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1070(t11589: f64, t567: f64, t4: f64, t4054: f64, t4048: f64, t431: f64, t122: f64, t457: f64, t4882: f64, t1303: f64, t521: f64, t1338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t13646 = t11589 * pi * t567;
    let t13654 = t4054 * t4;
    let t13675 = t431 * t4048;
    let t13676 = t13675 * t122;
    let t13679 = t4054 * pi * t457;
    let t13736 = t4882 * t122;
    let t13738 = t521 * t1303;
    let t13790 = t521 * t1338;
    (t13646, t13654, t13675, t13676, t13679, t13736, t13738, t13790)
}
