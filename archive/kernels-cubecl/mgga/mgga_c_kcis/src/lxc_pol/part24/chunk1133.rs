//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1133/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1133<F: Float>(t26544: F, t8531: F, t113: F, t9149: F, t2588: F, t26530: F, t157: F, t62: F, t9161: F, t7624: F, t8755: F, t36222: F, t808: F) -> (F, F, F, F, F, F) {
    let t91828 = t8531 * t26544;
    let t91830 = t9149 * t113;
    let t91832 = t2588 * t26530;
    let t91835 = t157 * t62 * t9161;
    let t91837 = t8755 * t7624;
    let t91839 = t808 * t36222;
    (t91828, t91830, t91832, t91835, t91837, t91839)
}
