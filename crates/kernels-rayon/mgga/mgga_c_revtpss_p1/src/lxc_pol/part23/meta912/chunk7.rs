//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2939/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2939(t63533: f64, t63538: f64, t63541: f64, t63543: f64, t63545: f64, t63547: f64, t63549: f64, t63551: f64, t77829: f64, t77832: f64, t77835: f64, t77838: f64) -> f64 {
    let t78075 = -0.91285185185185185184e-1_f64 * t63533 + 0.5477111111111111111e0_f64 * t63538 - 0.98587999999999999998e0_f64 * t77829 + 0.49293999999999999999e0_f64 * t77832 - 0.82156666666666666668e-1_f64 * t77835 - 0.82156666666666666668e-1_f64 * t77838 - 0.32862666666666666666e0_f64 * t63541 + 0.5477111111111111111e-1_f64 * t63543 - 0.27385555555555555555e0_f64 * t63545 - 0.32862666666666666666e0_f64 * t63547 + 0.10954222222222222222e0_f64 * t63549 + 0.73028148148148148146e-1_f64 * t63551;
    t78075
}
