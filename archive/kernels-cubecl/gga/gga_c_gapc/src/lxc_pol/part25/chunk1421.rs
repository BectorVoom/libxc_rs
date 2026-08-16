//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1421/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1421<F: Float>(t34949: F, t34954: F, t34962: F, t37134: F, t37135: F, t37136: F, t37138: F, t37140: F, t37141: F, t37142: F, t37144: F, t35034: F, t35041: F, t37172: F, t37174: F, t37175: F, t37177: F, t37178: F, t37179: F, t37180: F, t37181: F, t37182: F) -> (F, F) {
    let t38637 = t37134 - t37135 + t37136 - F::cast_from(0.18115908419564701086e-6_f64) * t34949 - t37138 - F::cast_from(0.98380106748709416171e-8_f64) * t34954 - t37140 - t37141 - t37142 - F::cast_from(0.18115908419564701086e-6_f64) * t34962 - t37144;
    let t38647 = -t37172 - F::cast_from(0.4419852458519115466e-7_f64) * t35034 - t37174 - t37175 - F::cast_from(0.57970906942607043475e-5_f64) * t35041 - t37177 - t37178 - t37179 + t37180 + t37181 - t37182;
    (t38637, t38647)
}
