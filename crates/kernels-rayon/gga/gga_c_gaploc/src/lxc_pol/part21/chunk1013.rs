//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1013/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1013(t10862: f64, t10864: f64, t10866: f64, t10869: f64, t10871: f64, t10873: f64, t9812: f64, t9814: f64, t9822: f64, t9826: f64, t9831: f64, t9835: f64, t9837: f64, t9845: f64, t9848: f64) -> f64 {
    let t12199 = -t10862 + t10864 + t10866 - t10869 + t10871 + t10873 + t9812 + 0.51123901271894332903e0_f64 * t9814 - t9822 + t9826 + 0.38342925953920749677e0_f64 * t9831 - 0.85206502119823888171e-1_f64 * t9835 + 0.38342925953920749677e0_f64 * t9837 - 0.38342925953920749677e0_f64 * t9845 - 0.38342925953920749677e0_f64 * t9848;
    t12199
}
