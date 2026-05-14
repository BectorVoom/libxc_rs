//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1279/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1279<F: Float>(t1869: F, t1894: F, t34159: F, t7405: F, t112182: F, t112216: F, t112244: F, t116133: F, t116139: F, t116147: F, t116150: F, t116153: F, t116156: F, t32942: F, t33005: F, t33023: F, t34073: F, t34154: F, t34261: F, t9940: F) -> (F, F) {
    let t116161 = t1869 * t34159 * t7405 * t1894;
    let t116165 = 0.14739506172839506172e-2 * t112182 - 0.44218518518518518517e-2 * t116133 + 0.10416666666666666667e-1 * t112216 * t9940 - 0.88437037037037037034e-2 * t116139 + 0.20833333333333333334e-1 * t112244 * t9940 + 0.20833333333333333334e-1 * t32942 * t34261 + 0.88437037037037037034e-2 * t116147 - t116150 + 0.33163888888888888888e-2 * t116153 + t116156 - 0.20833333333333333334e-1 * t34073 * t33023 + 0.99491666666666666664e-2 * t116161 - 0.120625e-1 * t34154 * t33005;
    (t116161, t116165)
}
