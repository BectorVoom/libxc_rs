//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 699/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk699(t2468: f64, t880: f64, t882: f64, t319: f64) -> (f64, f64, f64, f64) {
    let t7056 = t880 * t2468;
    let t7061 = t882 * t882;
    let t7062 = 1.0_f64 / t7061;
    let t7063 = t319 * t7062;
    (t7056, t7061, t7062, t7063)
}
