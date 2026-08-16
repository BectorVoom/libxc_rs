//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2129/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2129(t1497: f64, t5816: f64, t5872: f64, t1927: f64, t5825: f64, t1486: f64, t5819: f64, t22603: f64) -> (f64, f64, f64, f64, f64) {
    let t22656 = t5816 * t1497;
    let t22659 = t1497 * t5872;
    let t22662 = t1927 * t5825;
    let t22665 = t5819 * t1486;
    let t22670 = 6.0_f64 * t22603;
    (t22656, t22659, t22662, t22665, t22670)
}
