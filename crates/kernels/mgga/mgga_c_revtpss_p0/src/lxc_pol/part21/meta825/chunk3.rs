//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3076/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3076<F: Float>(t43858: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t56212: F, t56214: F, t56216: F, t56221: F, t56226: F, t56229: F, t56230: F, t56234: F, t56236: F, t56248: F, t56252: F, t56256: F) -> F {
    let t56258 = F::cast_from(0.61805555555555555556e-2_f64) * t56212 + F::cast_from(0.37083333333333333334e-1_f64) * t56214 - F::cast_from(0.10300925925925925926e-1_f64) * t56216 + F::cast_from(0.30902777777777777778e-1_f64) * t56221 + F::cast_from(0.55625000000000000001e-1_f64) * t56226 + t56229 - F::cast_from(0.92708333333333333334e-2_f64) * t56230 + F::cast_from(0.92708333333333333333e-2_f64) * t56234 - F::cast_from(0.96141975308641975309e-2_f64) * t56236 - F::cast_from(0.34336419753086419753e-2_f64) * t43858 - F::cast_from(0.82407407407407407408e-2_f64) * t43865 + F::cast_from(0.12361111111111111111e-1_f64) * t43883 - F::cast_from(0.28842592592592592593e-1_f64) * t43888 + F::cast_from(0.12361111111111111111e-1_f64) * t43890 + F::cast_from(0.24722222222222222222e-1_f64) * t43892 - F::cast_from(0.18541666666666666667e-1_f64) * t43894 - F::cast_from(0.30902777777777777778e-2_f64) * t43896 + F::cast_from(0.30902777777777777778e-1_f64) * t56248 + F::new(0.166875e0) * t56252 - F::new(0.11125e0) * t56256;
    t56258
}
