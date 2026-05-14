//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 947/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk947<F: Float>(t126: F, t190: F, t3044: F, t15542: F, t7953: F, t21801: F, t7259: F, t7325: F, t11799: F, t129: F, t18866: F, t11798: F, t28370: F, t7453: F, t19048: F, t3284: F) -> (F, F, F, F, F, F, F) {
    let t33287 = t126 * t190 * t3044;
    let t33289 = t7953 * t33287 * t15542;
    let t33291 = t7259 * t21801;
    let t33292 = t33291 * t7325;
    let t33295 = t18866 * t129 * t11799;
    let t33298 = t11798 * t28370 * t7453;
    let t33301 = t11798 * t3284 * t19048;
    (t33287, t33289, t33291, t33292, t33295, t33298, t33301)
}
