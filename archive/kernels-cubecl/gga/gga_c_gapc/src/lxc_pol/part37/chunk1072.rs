//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1072/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1072<F: Float>(t12628: F, t12653: F, t224: F, t3916: F, t987: F, t3707: F, t435: F, t1736: F, t474: F, t177: F, t208: F, t4913: F) -> (F, F, F, F, F, F) {
    let t12654 = t12628 + t12653;
    let t12655 = t224 * t12654;
    let t12667 = t987 * t3916;
    let t12744 = t435 * t3707;
    let t12768 = t474 * t1736;
    let t13281 = t177 / t4913 / t208;
    (t12654, t12655, t12667, t12744, t12768, t13281)
}
