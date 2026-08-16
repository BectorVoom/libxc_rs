//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 955/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk955(t114726: f64, t114740: f64, t23035: f64, t2379: f64, t31376: f64, t6637: f64, t114674: f64, t1888: f64, t232: f64, t6646: f64, t31386: f64, t6579: f64) -> (f64, f64, f64, f64) {
    let t114741 = t114726 + t114740;
    let t114746 = t23035 * t6637 * t31376 * t2379;
    let t114750 = t1888 * t6646 * t114674 * t232;
    let t114752 = t6579 * t31386;
    (t114741, t114746, t114750, t114752)
}
