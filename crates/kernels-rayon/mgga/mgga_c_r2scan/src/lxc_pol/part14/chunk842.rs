//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 842/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk842(t538: f64, t7619: f64, t6155: f64, t2162: f64, t503: f64, t113: f64, t2533: f64) -> (f64, f64, f64, f64) {
    let t7620 = t538 * t7619;
    let t7622 = 0.10975748638225852664e-1_f64 * t6155 * t7620;
    let t7623 = t503 * t2162;
    let t7624 = t2533 * t113;
    (t7620, t7622, t7623, t7624)
}
