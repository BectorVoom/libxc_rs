//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1068/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1068(t1980: f64, t7339: f64, t20157: f64, t805: f64, t831: f64, t5558: f64, t952: f64, t1959: f64, t2590: f64, t119: f64, t19077: f64, t481: f64) -> (f64, f64, f64, f64, f64) {
    let t23495 = t1980 * t7339;
    let t23516 = t805 * t831 * t20157;
    let t23555 = t952 * t5558;
    let t23575 = t2590 * t1959;
    let t23609 = t481 * t19077 * t119;
    (t23495, t23516, t23555, t23575, t23609)
}
