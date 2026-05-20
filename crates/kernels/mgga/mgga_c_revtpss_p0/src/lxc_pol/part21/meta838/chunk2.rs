//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3141/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3141<F: Float>(t56228: F, t43858: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t56212: F, t56214: F, t56216: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t56248: F, t56252: F, t56256: F) -> F {
    let t57889 = F::cast_from(0.23744444444444444444e-1_f64) * t56228;
    let t57904 = F::cast_from(0.11872222222222222222e-1_f64) * t56212 + F::cast_from(0.71233333333333333331e-1_f64) * t56214 - F::cast_from(0.19787037037037037036e-1_f64) * t56216 + F::cast_from(0.5936111111111111111e-1_f64) * t56221 + F::new(0.10685e0) * t56226 + t57889 - F::cast_from(0.17808333333333333333e-1_f64) * t56230 + F::cast_from(0.17808333333333333333e-1_f64) * t56234 - F::cast_from(0.18467901234567901234e-1_f64) * t56236 - F::cast_from(0.65956790123456790122e-2_f64) * t43858 - F::cast_from(0.15829629629629629629e-1_f64) * t43865 + F::cast_from(0.23744444444444444444e-1_f64) * t43883 - F::cast_from(0.55403703703703703702e-1_f64) * t43888 + F::cast_from(0.23744444444444444444e-1_f64) * t43890 + F::cast_from(0.47488888888888888887e-1_f64) * t43892 - F::cast_from(0.35616666666666666666e-1_f64) * t43894 - F::cast_from(0.5936111111111111111e-2_f64) * t43896 + F::cast_from(0.59361111111111111111e-1_f64) * t56248 + F::new(0.32055e0) * t56252 - F::cast_from(0.21369999999999999999e0_f64) * t56256;
    t57904
}
