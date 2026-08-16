//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 967/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk967<F: Float>(t77095: F, t14451: F, t5898: F, t26291: F, t71005: F, t74520: F, t74523: F, t74553: F, t77069: F, t77070: F, t77075: F, t77077: F, t77081: F, t77082: F, t77083: F, t77084: F, t77087: F, t77090: F, t77093: F) -> (F, F) {
    let t77096 = F::cast_from(0.17961362552795712846e0_f64) * t77095;
    let t77097 = t14451 * t5898;
    let t77098 = t26291 * t77097;
    let t77099 = F::cast_from(0.8980681276397856423e-1_f64) * t77098;
    let t77101 = t74520 + F::cast_from(0.82834157616596963776e-1_f64) * t74523 - t77069 + t77070 - t77075 - t77077 - t77081 + t77082 - t77083 + t77084 - t77087 - t77090 - t77093 + t77096 + t77099 - t71005 - F::cast_from(0.17451485956252114154e-4_f64) * t74553;
    (t77097, t77101)
}
