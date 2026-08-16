//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2569/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2569(t50846: f64, t51271: f64, t71146: f64, t71150: f64, t71152: f64, t71154: f64, t71156: f64, t71160: f64, t71166: f64, t71170: f64, t71174: f64, t71179: f64) -> f64 {
    let t71941 = -0.19128703703703703704e0_f64 * t71146 + 0.516475e0_f64 * t71150 - 0.103295e1_f64 * t71152 - 0.17215833333333333333e0_f64 * t71154 + 0.68863333333333333333e0_f64 * t71156 - 0.92617777777777777779e0_f64 * t50846 + t51271 + 0.17215833333333333334e1_f64 * t71160 - 0.15302962962962962963e1_f64 * t71166 + 0.929655e1_f64 * t71170 + 0.123954e2_f64 * t71174 + 0.103295e1_f64 * t71179;
    t71941
}
