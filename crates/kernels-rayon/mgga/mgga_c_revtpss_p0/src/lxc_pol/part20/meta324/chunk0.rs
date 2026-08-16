//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1236/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1236(t12966: f64, t480: f64, t12621: f64, t482: f64, t371: f64, t372: f64, t12657: f64, t225: f64) -> (f64, f64, f64, f64) {
    let t12967 = t12966 * t480;
    let t12970 = t482 * t12621;
    let t12972 = t371 * t372 * t12970;
    let t12975 = t12657 * t225;
    (t12967, t12970, t12972, t12975)
}
