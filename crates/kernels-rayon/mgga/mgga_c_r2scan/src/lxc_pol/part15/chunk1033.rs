//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1033/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1033(t2526: f64, t6212: f64, t19790: f64, t910: f64, t146: f64, t5094: f64, t774: f64, t560: f64, t7977: f64, t481: f64, t1234: f64, t2841: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24912 = t6212 * t2526;
    let t24916 = t19790 * t910;
    let t25169 = t146 * t5094 * t774;
    let t25172 = t7977 * t560;
    let t25177 = t7977 * t481;
    let t25183 = t2841 * t1234;
    (t24912, t24916, t25169, t25172, t25177, t25183)
}
