//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 715/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk715<F: Float>(t12664: F, t15362: F, t28594: F, t7785: F, t12705: F, t7416: F, t10012: F, t2530: F, t2684: F, t9438: F, t12657: F, t23157: F, t2365: F, t28652: F, t6111: F, t40820: F, t900: F) -> (F, F, F, F, F, F, F) {
    let t41305 = t15362 * t12664;
    let t41307 = t28594 * t7785;
    let t41312 = t7416 * t12705;
    let t41316 = t2684 * t9438 * t10012 * t2530;
    let t41330 = t23157 * t12657;
    let t41337 = t6111 * t2365 * t28652;
    let t41339 = t900 * t40820;
    (t41305, t41307, t41312, t41316, t41330, t41337, t41339)
}
