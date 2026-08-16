//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1193/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1193(t1: f64, t1736: f64, t2206: f64, t311: f64, t3383: f64, t8675: f64, t1038: f64, t28622: f64, t4043: f64, t6851: f64, t1026: f64, t1093: f64, t2153: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t34005 = t311 * t2206 * t1736 * t1;
    let t34007 = t34005 * t8675 * t3383;
    let t34013 = t311 * t6851 * t4043 * pi * t1038 * t28622;
    let t34016 = t2153 * t1026 * t1093;
    (t34005, t34007, t34013, t34016)
}
