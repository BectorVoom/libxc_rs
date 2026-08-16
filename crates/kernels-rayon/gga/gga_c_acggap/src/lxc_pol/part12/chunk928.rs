//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 928/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk928(t2019: f64, t2028: f64, t1152: f64, t1: f64, t2065: f64, t7335: f64, t1160: f64) -> (f64, f64, f64) {
    let t31110 = t2019 * t2028;
    let t31111 = t31110 * t1152;
    let t31114 = t2065 * t7335 * t1;
    let t31115 = t1160 * t31114;
    (t31111, t31114, t31115)
}
