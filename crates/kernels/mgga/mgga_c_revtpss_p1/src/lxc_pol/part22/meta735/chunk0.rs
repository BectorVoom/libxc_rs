//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2795/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2795<F: Float>(t10688: F, t243: F, t268: F, t40634: F, t2694: F, t9784: F, t10681: F, t2689: F, t16: F, t2236: F, t240: F, t236: F, t281: F, t39644: F) -> (F, F, F, F, F, F) {
    let t40638 = F::cast_from(0.53552153920316253184e-5_f64) * t10688 * t40634 * t243 * t268;
    let t40639 = t9784 * t2694;
    let t40645 = t2689 * t10681;
    let t40648 = t2236 * t16;
    let t40649 = F::cast_from(1.0_f64) / t40648;
    let t40650 = t40649 * t240;
    let t40654 = F::cast_from(0.47607864835161149081e-7_f64) * t39644 * t236 * t40650 * t243 * t281;
    (t40638, t40639, t40645, t40649, t40650, t40654)
}
