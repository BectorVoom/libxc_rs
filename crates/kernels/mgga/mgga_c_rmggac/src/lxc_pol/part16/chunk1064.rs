//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1064/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1064<F: Float>(t38382: F, t44694: F, t44696: F, t44700: F, t44702: F, t44705: F, t44724: F, t44727: F, t44734: F, t44738: F, t44740: F, t44744: F, t44748: F, t44752: F, t44755: F, t44759: F, t44763: F) -> F {
    let t48176 = -F::cast_from(0.1702583995731913576e-4_f64) * t44694 - F::cast_from(0.1702583995731913576e-4_f64) * t44696 + F::cast_from(0.212822999466489197e-4_f64) * t44700 - F::cast_from(0.16364796992547205038e0_f64) * t44702 + F::cast_from(0.2727466165424534173e0_f64) * t44705 + F::cast_from(0.17961362552795712846e0_f64) * t44724 + F::cast_from(0.5987120850931904282e-1_f64) * t44727 + F::cast_from(0.43639458646792546768e0_f64) * t44734 + F::cast_from(0.10909864661698136692e0_f64) * t44738 - F::cast_from(0.1489760996265424379e-3_f64) * t44740 - F::cast_from(0.1702583995731913576e-4_f64) * t44744 + F::cast_from(0.5107751987195740728e-4_f64) * t44748 + F::cast_from(0.5107751987195740728e-4_f64) * t44752 - F::cast_from(0.5107751987195740728e-4_f64) * t44755 + F::cast_from(0.58540737209111952978e0_f64) * t38382 - F::cast_from(0.212822999466489197e-4_f64) * t44759 - F::cast_from(0.5107751987195740728e-4_f64) * t44763;
    t48176
}
