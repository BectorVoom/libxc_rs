//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 283/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk283(t40: f64, t607: f64, t767: f64, t76: f64, zeta_threshold: f64) -> (f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t770 = piecewise3(t146, 0.0_f64, 2.0_f64 / 3.0_f64 * t767 * t607);
    let t771 = 1.0_f64 / t76;
    (t770, t771)
}
