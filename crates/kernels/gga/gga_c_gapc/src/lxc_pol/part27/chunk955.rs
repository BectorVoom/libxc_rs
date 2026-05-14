//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 955/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk955<F: Float>(t33387: F, t33390: F, t33394: F, t33396: F, t33402: F, t33405: F, t33407: F, t33409: F, t33413: F, t33417: F, t33420: F, t11387: F, t15650: F, t7204: F, t8785: F, t8910: F) -> (F, F, F) {
    let t33422 = -0.38647271295071362318e-6 * t33387 + 0.33764099580923002116e-6 * t33390 - 0.4976888445083044254e-7 * t33394 - 0.52756405595192190805e-8 * t33396 + 0.22098551499687900009e-8 * t33402 - 0.21102562238076876322e-7 * t33405 - 0.18115908419564701086e-6 * t33407 + 0.52756405595192190805e-8 * t33409 + 0.168651611569216142e-8 * t33413 + 0.27665946779727057415e-8 * t33417 - 0.33147827249531850014e-7 * t33420;
    let t33427 = t7204 * t11387 * t15650;
    let t33429 = t8910 * t8785;
    (t33422, t33427, t33429)
}
