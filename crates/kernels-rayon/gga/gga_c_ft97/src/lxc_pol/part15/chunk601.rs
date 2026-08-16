//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 601/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk601(t10397: f64, t192: f64, t7640: f64, t869: f64, t309: f64, t2770: f64, t871: f64, t313: f64, t89: f64, t9555: f64, t295: f64, t9568: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10658 = 28.0_f64 / 81.0_f64 * t10397;
    let t10683 = t192 * t7640;
    let t10695 = t869 * t869;
    let t10696 = 1.0_f64 / t10695;
    let t10697 = t309 * t10696;
    let t10703 = t2770 * t871;
    let t10749 = 28.0_f64 / 81.0_f64 * t89 * t9555 * t313;
    let t10758 = t9568 * t295;
    (t10658, t10683, t10695, t10696, t10697, t10703, t10749, t10758)
}
