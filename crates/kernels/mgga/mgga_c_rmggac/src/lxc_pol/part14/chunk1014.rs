//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1014/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1014<F: Float>(t41047: F, t854: F, t41031: F, t797: F, t36094: F, t36096: F, t36099: F, t36101: F, t36115: F, t36117: F, t41234: F, t41235: F, t41237: F, t41239: F, t41242: F, t41243: F, t41245: F, t41247: F) -> F {
    let t41255 = t854 * t41047;
    let t41257 = t797 * t41031;
    let t41259 = -t41234 + F::new(0.19914231157590872008e-2) * t41235 - F::new(0.27879923620627220811e-2) * t41237 + F::new(0.19914231157590872008e-2) * t41239 + t41242 - F::new(0.26552308210121162678e-2) * t41243 + F::new(0.39828462315181744016e-2) * t41245 + F::new(0.38943385374844371927e-2) * t41247 + F::new(0.66671395154821946449e-1) * t36094 - F::new(0.88895193539762595266e-1) * t36096 + F::new(0.28224120208536198847e-3) * t36099 - F::new(0.90915538847484472431e-2) * t36101 + F::new(0.33868944250243438616e-2) * t36115 + F::new(0.72732431077987577945e-1) * t36117 + F::new(0.39828462315181744016e-3) * t41255 + F::new(0.14635184302277988245e0) * t41257;
    t41259
}
