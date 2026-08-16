//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 185/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk185(t688: f64, t690: f64, t694: f64, t699: f64, t180: f64) -> (f64, f64) {
    let t745 = -0.86308333333333333334e0_f64 * t688 - 0.301925e0_f64 * t690 - 0.5501625e-1_f64 * t694 - 0.82785e-1_f64 * t699;
    let t746 = 1.0_f64 / t180;
    (t745, t746)
}
