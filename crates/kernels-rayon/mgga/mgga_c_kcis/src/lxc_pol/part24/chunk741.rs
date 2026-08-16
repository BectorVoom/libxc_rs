//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 741/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk741(t647: f64, t9129: f64, t794: f64, t8604: f64, t103: f64, t11: f64, t144: f64, t148: f64, t2477: f64, t2542: f64, t2546: f64, t2555: f64, t2561: f64, t2565: f64, t2569: f64, t745: f64, t784: f64, t791: f64, t795: f64, t85: f64, t8996: f64, t9113: f64, t9118: f64, t9120: f64, t9124: f64) -> f64 {
    let t9130 = t9129 * t647;
    let t9144 = t794 * t8604;
    let t9147 = 0.74295e-1_f64 * t9113 * t2561 + 0.4953e-1_f64 * t2546 * t2565 - 0.619125e-2_f64 * t9118 * t9120 - 0.371475e-1_f64 * t9124 * t791 + 0.371475e-1_f64 * t784 * t2569 + 0.619125e-2_f64 * t9130 * t2555 - 0.79593333333333333331e-1_f64 * t85 * t148 * t8996 + 0.5306222222222222222e-1_f64 * t85 * t103 * t745 - 0.15918666666666666666e0_f64 * t85 * t11 * t2477 - 0.1857375e-1_f64 * t2542 * t795 - 0.619125e-2_f64 * t144 * t9144;
    t9147
}
