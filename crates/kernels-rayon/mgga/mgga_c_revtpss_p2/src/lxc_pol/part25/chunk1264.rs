//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1264/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1264(t2439: f64, t7048: f64, t780: f64, t785: f64, t25310: f64, t25331: f64, t25412: f64, t93329: f64, t25411: f64, t25431: f64, t2435: f64, t25339: f64) -> (f64, f64, f64, f64, f64) {
    let t93382 = t2439 * t785 * t7048 * t780;
    let t93384 = t25310 * t25331;
    let t93386 = t93329 * t25412;
    let t93387 = t25411 * t93386;
    let t93389 = t25431 * t93386;
    let t93391 = t2435 * t25339;
    (t93382, t93384, t93387, t93389, t93391)
}
