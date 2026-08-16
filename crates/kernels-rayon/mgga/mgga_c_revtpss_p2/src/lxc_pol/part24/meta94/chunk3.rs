//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 543/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk543(t2986: f64, t315: f64, t2846: f64, t2904: f64, t963: f64) -> (f64, f64, f64, f64) {
    let t2987 = t315 * t2986;
    let t2994 = 0.40256666666666666667e0_f64 * t2846;
    let t3001 = 0.137975e0_f64 * t2904;
    let t3010 = t963 * t963;
    (t2987, t2994, t3001, t3010)
}
