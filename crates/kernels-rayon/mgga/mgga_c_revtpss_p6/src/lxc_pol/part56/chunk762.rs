//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 762/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk762(t2148: f64, t8931: f64, t2150: f64, t473: f64, t2147: f64, t456: f64, t3565: f64) -> (f64, f64, f64, f64) {
    let t8932 = t2148 * t8931;
    let t8933 = t2150 * t473;
    let t8936 = t2147 * t456;
    let t8937 = t8936 * t3565;
    (t8932, t8933, t8936, t8937)
}
