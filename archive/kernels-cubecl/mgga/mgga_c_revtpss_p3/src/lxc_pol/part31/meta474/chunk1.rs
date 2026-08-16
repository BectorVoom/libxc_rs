//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1741/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1741<F: Float>(t10301: F, t6957: F, t38: F, t6972: F, t2247: F, t48: F, t613: F, t2275: F, t43: F, t239: F, t10309: F, t607: F) -> (F, F, F, F, F, F, F, F) {
    let t25099 = t10301 * t6957;
    let t25105 = t38 * t6972;
    let t25106 = t2247 * t25105;
    let t25129 = t613 * t48;
    let t25132 = t43 * t2275;
    let t25137 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t239;
    let t25157 = t10309 * t6957;
    let t25162 = t2247 * t607;
    (t25099, t25105, t25106, t25129, t25132, t25137, t25157, t25162)
}
