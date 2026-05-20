//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1477/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1477<F: Float>(t3670: F, t6594: F, t3718: F, t44546: F, t6689: F, t3717: F, t70994: F, t3617: F, t6587: F, t3147: F, t6593: F, t3594: F, t3597: F) -> (F, F, F, F, F, F) {
    let t71280 = t3670 * t6594;
    let t71294 = t3718 * t44546 * t6689;
    let t71513 = t3717 * t70994;
    let t71543 = t3617 * t6587;
    let t71691 = t6593 * t3147;
    let t71693 = t3594 * t3597 * t71691;
    (t71280, t71294, t71513, t71543, t71691, t71693)
}
