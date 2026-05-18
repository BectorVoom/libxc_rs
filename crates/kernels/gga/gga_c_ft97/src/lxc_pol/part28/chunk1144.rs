//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1144/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1144<F: Float>(t34927: F, t376: F, t89: F, t148132: F, t32906: F, t7239: F, t7366: F, t3526: F, t7312: F, t32888: F, t7369: F, t147656: F, t446: F, t9073: F) -> (F, F, F, F, F) {
    let t148511 = t89 * t376 * t34927;
    let t148515 = t7366 * t7239 * t32906 * t148132;
    let t148517 = t7312 * t3526;
    let t148520 = t32888 * t7239 * t7369 * t148517;
    let t148523 = t446 * t9073 * t147656;
    (t148511, t148515, t148517, t148520, t148523)
}
