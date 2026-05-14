//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1148/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1148<F: Float>(t33239: F, t2009: F, t2021: F, t2028: F, t28529: F, t33205: F, t33206: F, t33210: F, t33212: F, t33215: F, t33218: F, t33221: F, t33223: F, t33225: F, t33228: F, t33231: F, t33232: F, t33238: F) -> (F,) {
    let t33240 = 0.59584149919750711116e-1 * t33239;
    let t33241 = -t33205 - 0.79445533226334281486e-1 * t33206 * t2028 - t33210 - t33212 + t28529 + t33215 - t33218 + t33221 - t33223 - t33225 + t33228 - t33231 - 0.71500979903700853338e0 * t2021 * t33232 * t2009 - t33238 - t33240;
    (t33241,)
}
