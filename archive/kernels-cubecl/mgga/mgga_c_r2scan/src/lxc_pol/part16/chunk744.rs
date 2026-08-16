//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 744/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk744<F: Float>(t495: F, t6212: F, t6211: F, t6209: F, t2182: F, t489: F, t548: F, t2090: F, t57: F, t128: F, t524: F, t540: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6213 = t6212 * t495;
    let t6214 = t6211 * t6213;
    let t6215 = t6209 * t6214;
    let t6217 = t2182 * t489;
    let t6218 = t6217 * t548;
    let t6238 = t2090 * t57;
    let t6239 = t6238 * t128;
    let t6240 = t524 * t6239;
    let t6241 = t6240 * t540;
    (t6213, t6214, t6215, t6217, t6218, t6238, t6239, t6240, t6241)
}
