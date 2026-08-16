//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 779/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk779(t7383: f64, t969: f64, t825: f64, t2685: f64, t2684: f64, t2021: f64, t2032: f64) -> (f64, f64, f64) {
    let t7384 = t969 * t7383;
    let t7385 = t825 * t7384;
    let t7387 = t2685 * t7383;
    let t7388 = t2684 * t7387;
    let t7390 = t2021 * t2032;
    (t7385, t7388, t7390)
}
