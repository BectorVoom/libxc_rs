//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1795/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1795(t25972: f64, t3981: f64, t2019: f64, t3985: f64, t7269: f64, t820: f64, t843: f64) -> (f64, f64, f64) {
    let t25973 = t25972 * t3981;
    let t25975 = t2019 * t3985;
    let t25976 = 0.11337795902333997111e-1_f64 * t25975;
    let t25978 = t820 * t7269 * t843;
    (t25973, t25976, t25978)
}
