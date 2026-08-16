//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1815/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1815(t3920: f64, t7246: f64, t2023: f64, t2453: f64, t3908: f64, t72: f64, t7307: f64, t686: f64, t7284: f64, t1426: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26040 = 0.13009920719177044025e-1_f64 * t7246 * t3920;
    let t26041 = t2453 * t2023;
    let t26043 = 0.11565819519348392139e-2_f64 * t26041 * t3908;
    let t26049 = t7307 * t72;
    let t26050 = t26049 * t686;
    let t26051 = t7284 * t26050;
    let t26053 = t2023 * t1426;
    let t26054 = t786 * t26053;
    (t26040, t26041, t26043, t26049, t26050, t26051, t26053, t26054)
}
