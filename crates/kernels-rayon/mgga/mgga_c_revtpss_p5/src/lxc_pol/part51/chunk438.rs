//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 438/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk438(t143: f64, t680: f64, t130: f64, t700: f64, t701: f64) -> (f64, f64) {
    let t2563 = t680 * t143;
    let t2564 = 1.0_f64 / t2563;
    let t2565 = t130 * t2564;
    let t2566 = t700 * t700;
    let t2567 = t2566 * t701;
    let t2569 = 2.0_f64 * t2565 * t2567;
    (t2566, t2569)
}
