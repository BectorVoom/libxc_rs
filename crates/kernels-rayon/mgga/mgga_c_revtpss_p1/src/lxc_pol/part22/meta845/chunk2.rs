//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2982/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2982(t10069: f64, t14225: f64, t10013: f64, t14224: f64, t2782: f64, t48073: f64, t543: f64, t4100: f64, t4086: f64, t49213: f64, t10136: f64, t14114: f64) -> (f64, f64, f64, f64, f64) {
    let t49289 = t10069 * t14225;
    let t49296 = t2782 * t10013 * t14224;
    let t49306 = t48073 * t543;
    let t49308 = t2782 * t4100 * t49306;
    let t49313 = t2782 * t4086 * t49213 * t543;
    let t49321 = t14114 * t10136;
    (t49289, t49296, t49308, t49313, t49321)
}
