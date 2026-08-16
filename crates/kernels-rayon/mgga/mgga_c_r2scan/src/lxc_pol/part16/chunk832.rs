//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 832/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk832(t2551: f64, t8735: f64, t5109: f64, t277: f64, t3190: f64, t495: f64, t360: f64, t3052: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8769 = t8735 * t2551;
    let t8770 = t5109 * t8769;
    let t8773 = t277 * t3190;
    let t8774 = t8773 * t495;
    let t8775 = t360 * t8774;
    let t8778 = t277 * t3052;
    (t8769, t8770, t8773, t8774, t8775, t8778)
}
