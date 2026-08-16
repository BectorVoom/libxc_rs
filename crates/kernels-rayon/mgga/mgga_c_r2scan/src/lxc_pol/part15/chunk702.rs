//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 702/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk702(t1727: f64, t591: f64, t171: f64, t5397: f64, t21: f64, t502: f64, t1684: f64, t1680: f64, t1678: f64, t607: f64, t159: f64, t1686: f64) -> (f64, f64, f64) {
    let t5398 = t1727 * t591;
    let t5401 = 0.6858336e0_f64 * t5397 * t171 * t5398;
    let t5402 = t21 * t502;
    let t5403 = t1684 * t5402;
    let t5405 = 0.16936279733333333332e-2_f64 * t1680 * t5403;
    let t5407 = t607 * t1678;
    let t5408 = t159 * t5407;
    let t5409 = t5408 * t1686;
    (t5401, t5405, t5409)
}
