//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 754/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk754(t3121: f64, t8636: f64, t1734: f64, t1903: f64, t1743: f64, t1912: f64, t129: f64, t5856: f64, t197: f64, t5858: f64, t1878: f64, t2986: f64) -> (f64, f64, f64, f64, f64) {
    let t8637 = t3121 * t8636;
    let t8639 = t1734 * t1903;
    let t8641 = t1743 * t8639 * t1912;
    let t8643 = t5856 * t129;
    let t8644 = t197 * t5858;
    let t8645 = t8643 * t8644;
    let t8647 = t2986 * t1878;
    (t8637, t8639, t8641, t8645, t8647)
}
