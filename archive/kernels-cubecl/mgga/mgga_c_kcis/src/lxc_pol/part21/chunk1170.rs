//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1170/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1170<F: Float>(t28041: F, t28043: F, t28046: F, t28048: F, t28051: F, t28053: F, t28055: F, t28057: F, t28060: F, t28062: F, t28064: F, t28066: F, t28068: F) -> F {
    let t28294 = F::cast_from(0.26979166666666666667e-1_f64) * t28041 + F::cast_from(0.1875e0_f64) * t28043 + F::cast_from(0.625e-1_f64) * t28046 - F::cast_from(0.26979166666666666667e-1_f64) * t28048 - F::cast_from(0.9375e-1_f64) * t28051 - F::cast_from(0.9375e-1_f64) * t28053 + F::cast_from(0.10791666666666666667e0_f64) * t28055 - F::cast_from(0.16666666666666666667e0_f64) * t28057 - F::cast_from(0.9375e-1_f64) * t28060 + F::cast_from(0.20234375e-1_f64) * t28062 + F::cast_from(0.20234375e-1_f64) * t28064 + F::cast_from(0.25e0_f64) * t28066 - F::cast_from(0.625e-1_f64) * t28068;
    t28294
}
