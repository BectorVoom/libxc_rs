//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 213/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk213(t249: f64, t787: f64, t803: f64, t805: f64, t809: f64, t817: f64, t831: f64, t840: f64, t843: f64, t849: f64) -> f64 {
    let t852 = -t803 - t787 * t805 / 48.0_f64 + t809 * t249 / 3072.0_f64 - t817 * t831 / 3072.0_f64 - t840 - t843 * t849 / 768.0_f64;
    t852
}
