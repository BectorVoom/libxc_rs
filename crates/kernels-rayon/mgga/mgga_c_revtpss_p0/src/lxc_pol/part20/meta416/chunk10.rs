//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1557/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1557(t39456: f64, t3376: f64, t3383: f64, t3386: f64, t3494: f64, t3519: f64, t3497: f64) -> (f64, f64, f64, f64) {
    let t43744 = -t39456;
    let t43748 = t3376 * t3383;
    let t43750 = 12.0_f64 * t43748 * t3386;
    let t43752 = 1.0_f64 / t3519 / t3494;
    let t43753 = t3497 * t3497;
    (t43744, t43750, t43752, t43753)
}
