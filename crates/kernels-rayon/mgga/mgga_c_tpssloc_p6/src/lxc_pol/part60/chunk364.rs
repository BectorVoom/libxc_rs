//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 364/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk364(t2225: f64, t594: f64, t598: f64, t15: f64) -> (f64, f64, f64, f64) {
    let t2226 = 0.778e2_f64 * t2225;
    let t2228 = 0.16272e3_f64 * t594 * t598;
    let t2229 = t15 * t15;
    let t2230 = 1.0_f64 / t2229;
    (t2226, t2228, t2229, t2230)
}
