//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 967/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk967(t77095: f64, t14451: f64, t5898: f64, t26291: f64, t71005: f64, t74520: f64, t74523: f64, t74553: f64, t77069: f64, t77070: f64, t77075: f64, t77077: f64, t77081: f64, t77082: f64, t77083: f64, t77084: f64, t77087: f64, t77090: f64, t77093: f64) -> (f64, f64) {
    let t77096 = 0.17961362552795712846e0_f64 * t77095;
    let t77097 = t14451 * t5898;
    let t77098 = t26291 * t77097;
    let t77099 = 0.8980681276397856423e-1_f64 * t77098;
    let t77101 = t74520 + 0.82834157616596963776e-1_f64 * t74523 - t77069 + t77070 - t77075 - t77077 - t77081 + t77082 - t77083 + t77084 - t77087 - t77090 - t77093 + t77096 + t77099 - t71005 - 0.17451485956252114154e-4_f64 * t74553;
    (t77097, t77101)
}
