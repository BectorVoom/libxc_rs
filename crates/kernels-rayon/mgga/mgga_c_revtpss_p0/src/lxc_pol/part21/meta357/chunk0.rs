//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1710/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1710(t3154: f64, t999: f64, t11659: f64, t3117: f64, t1086: f64, t3046: f64, t3090: f64) -> (f64, f64, f64, f64) {
    let t11860 = t3154 * t999;
    let t11861 = t11659 * t11860;
    let t11862 = t3117 * t11861;
    let t11865 = t3046 * t1086;
    let t11866 = t11865 * t3090;
    (t11861, t11862, t11865, t11866)
}
