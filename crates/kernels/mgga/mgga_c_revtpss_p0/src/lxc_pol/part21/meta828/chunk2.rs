//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3086/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3086<F: Float>(t56228: F, t43858: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t56212: F, t56214: F, t56216: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t56248: F, t56252: F, t56256: F) -> F {
    let t56462 = F::cast_from(0.11111111111111111111e-1_f64) * t56228;
    let t56477 = F::cast_from(0.55555555555555555555e-2_f64) * t56212 + F::cast_from(0.33333333333333333333e-1_f64) * t56214 - F::cast_from(0.92592592592592592593e-2_f64) * t56216 + F::cast_from(0.27777777777777777777e-1_f64) * t56221 + F::cast_from(0.50000000000000000001e-1_f64) * t56226 + t56462 - F::cast_from(0.83333333333333333334e-2_f64) * t56230 + F::cast_from(0.83333333333333333333e-2_f64) * t56234 - F::cast_from(0.86419753086419753086e-2_f64) * t56236 - F::cast_from(0.30864197530864197532e-2_f64) * t43858 - F::cast_from(0.74074074074074074074e-2_f64) * t43865 + F::cast_from(0.11111111111111111111e-1_f64) * t43883 - F::cast_from(0.25925925925925925926e-1_f64) * t43888 + F::cast_from(0.11111111111111111111e-1_f64) * t43890 + F::cast_from(0.22222222222222222222e-1_f64) * t43892 - F::cast_from(0.16666666666666666667e-1_f64) * t43894 - F::cast_from(0.27777777777777777778e-2_f64) * t43896 + F::cast_from(0.27777777777777777777e-1_f64) * t56248 + F::new(0.15e0) * t56252 - F::cast_from(0.99999999999999999999e-1_f64) * t56256;
    t56477
}
