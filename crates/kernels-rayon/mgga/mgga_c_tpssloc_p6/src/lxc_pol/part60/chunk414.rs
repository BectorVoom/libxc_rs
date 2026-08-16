//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 414/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk414(t3787: f64, t562: f64, t193: f64, t532: f64, t531: f64, t571: f64, t111: f64, t576: f64) -> (f64, f64, f64, f64) {
    let t3897 = t3787 * t562;
    let t3918 = t193 * t532;
    let t3924 = t531 * t571;
    let t3941 = t576 * t111;
    (t3897, t3918, t3924, t3941)
}
