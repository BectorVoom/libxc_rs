//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3141/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3141(t56228: f64, t43858: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t56212: f64, t56214: f64, t56216: f64, t56221: f64, t56226: f64, t56230: f64, t56234: f64, t56236: f64, t56248: f64, t56252: f64, t56256: f64) -> f64 {
    let t57889 = 0.23744444444444444444e-1_f64 * t56228;
    let t57904 = 0.11872222222222222222e-1_f64 * t56212 + 0.71233333333333333331e-1_f64 * t56214 - 0.19787037037037037036e-1_f64 * t56216 + 0.5936111111111111111e-1_f64 * t56221 + 0.10685e0_f64 * t56226 + t57889 - 0.17808333333333333333e-1_f64 * t56230 + 0.17808333333333333333e-1_f64 * t56234 - 0.18467901234567901234e-1_f64 * t56236 - 0.65956790123456790122e-2_f64 * t43858 - 0.15829629629629629629e-1_f64 * t43865 + 0.23744444444444444444e-1_f64 * t43883 - 0.55403703703703703702e-1_f64 * t43888 + 0.23744444444444444444e-1_f64 * t43890 + 0.47488888888888888887e-1_f64 * t43892 - 0.35616666666666666666e-1_f64 * t43894 - 0.5936111111111111111e-2_f64 * t43896 + 0.59361111111111111111e-1_f64 * t56248 + 0.32055e0_f64 * t56252 - 0.21369999999999999999e0_f64 * t56256;
    t57904
}
