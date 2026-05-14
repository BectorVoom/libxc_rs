//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 858/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk858<F: Float>(t2301: F, t5245: F, t2295: F, t30510: F, t40883: F, t5259: F, t25820: F, t38977: F, t27101: F, t38980: F, t25854: F, t38983: F, t6444: F, t9005: F, t40134: F, t39700: F, t797: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41463 = t5245 * t2301;
    let t41475 = t30510 * t2295;
    let t41477 = t5259 * t40883;
    let t41488 = t25820 * t38977;
    let t41490 = t27101 * t38980;
    let t41492 = t25854 * t38983;
    let t41501 = t6444 * t9005;
    let t41506 = t5259 * t40134;
    let t41523 = t797 * t39700;
    (t41463, t41475, t41477, t41488, t41490, t41492, t41501, t41506, t41523)
}
