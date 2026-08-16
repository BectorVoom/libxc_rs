//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 892/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk892<F: Float>(t9468: F, t9474: F, t9478: F, t9481: F, t9483: F, t9486: F, t9488: F, t9491: F, t9494: F, t9499: F, t9502: F, t9505: F, t9509: F) -> F {
    let t10842 = -F::cast_from(0.2471588561924985691e-3_f64) * t9468 - F::cast_from(0.82386285397499523032e-5_f64) * t9474 + F::cast_from(0.6746961805555555556e-5_f64) * t9478 - F::cast_from(0.4637672555408563478e-4_f64) * t9481 - F::cast_from(0.21642471925239962898e-3_f64) * t9483 - F::cast_from(0.11254699860307667372e-6_f64) * t9486 + F::cast_from(0.55603792169291016668e-2_f64) * t9488 - F::cast_from(0.20240885416666666668e-4_f64) * t9491 - F::cast_from(0.20240885416666666668e-4_f64) * t9494 - F::cast_from(0.22202903123154399017e-4_f64) * t9499 + F::cast_from(0.11272120794395814009e-6_f64) * t9502 - F::cast_from(0.20041830772435757309e-6_f64) * t9505 + F::cast_from(0.55603792169291016668e-2_f64) * t9509;
    t10842
}
