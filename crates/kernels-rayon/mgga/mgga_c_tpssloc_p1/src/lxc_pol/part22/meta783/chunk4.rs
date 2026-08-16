//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2683/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2683(t16081: f64, t20586: f64, t40422: f64, t54663: f64, t54668: f64, t54676: f64, t54702: f64, t54725: f64, t56535: f64, t56537: f64, t56539: f64, t56548: f64, t56550: f64) -> f64 {
    let t74756 = t16081 * t20586;
    let t74765 = t40422 - 0.59999999999999999998e-1_f64 * t54663 + t54668 - 0.34999999999999999998e-1_f64 * t74756 + 0.47499999999999999998e-1_f64 * t56535 + 0.11666666666666666666e-1_f64 * t56537 - 0.15833333333333333333e-1_f64 * t56539 + 0.47499999999999999999e-1_f64 * t54676 + t54702 + 0.13999999999999999999e0_f64 * t56548 - 0.69999999999999999996e-1_f64 * t56550 + 0.8333333333333333333e-3_f64 * t54725;
    t74765
}
