//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2902/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2902(t51890: f64, t51892: f64, t51894: f64, t51896: f64, t51899: f64, t51902: f64, t51907: f64, t51909: f64, t51911: f64, t51913: f64, t51915: f64, t51917: f64) -> f64 {
    let t52677 = -0.28483875e1_f64 * t51890 - 0.9494625e0_f64 * t51892 + 0.46074375e0_f64 * t51894 + 0.15358125e0_f64 * t51896 - 0.3560484375e1_f64 * t51899 + 0.1151859375e0_f64 * t51902 - 0.82156666666666666668e-1_f64 * t51907 - 0.65725333333333333332e0_f64 * t51909 + 0.10954222222222222222e0_f64 * t51911 + 0.5477111111111111111e0_f64 * t51913 - 0.91285185185185185185e-1_f64 * t51915 - 0.32862666666666666666e0_f64 * t51917;
    t52677
}
