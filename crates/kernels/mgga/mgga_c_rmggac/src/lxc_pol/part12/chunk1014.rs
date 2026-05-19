//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1014/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1014<F: Float>(t41047: F, t854: F, t41031: F, t797: F, t36094: F, t36096: F, t36099: F, t36101: F, t36115: F, t36117: F, t41234: F, t41235: F, t41237: F, t41239: F, t41242: F, t41243: F, t41245: F, t41247: F) -> F {
    let t41255 = t854 * t41047;
    let t41257 = t797 * t41031;
    let t41259 = -t41234 + F::cast_from(0.19914231157590872008e-2_f64) * t41235 - F::cast_from(0.27879923620627220811e-2_f64) * t41237 + F::cast_from(0.19914231157590872008e-2_f64) * t41239 + t41242 - F::cast_from(0.26552308210121162678e-2_f64) * t41243 + F::cast_from(0.39828462315181744016e-2_f64) * t41245 + F::cast_from(0.38943385374844371927e-2_f64) * t41247 + F::cast_from(0.66671395154821946449e-1_f64) * t36094 - F::cast_from(0.88895193539762595266e-1_f64) * t36096 + F::cast_from(0.28224120208536198847e-3_f64) * t36099 - F::cast_from(0.90915538847484472431e-2_f64) * t36101 + F::cast_from(0.33868944250243438616e-2_f64) * t36115 + F::cast_from(0.72732431077987577945e-1_f64) * t36117 + F::cast_from(0.39828462315181744016e-3_f64) * t41255 + F::cast_from(0.14635184302277988245e0_f64) * t41257;
    t41259
}
