//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 360/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk360(t1708: f64, t85: f64, t87: f64, t40: f64, t495: f64) -> (f64, f64, f64, f64) {
    let t1709 = t1708 * t85;
    let t1710 = 0.19751673498613801407e-1_f64 * t1709;
    let t1711 = t1708 * t87;
    let t1712 = t40 * t1711;
    let t1713 = t495 * t495;
    (t1710, t1711, t1712, t1713)
}
