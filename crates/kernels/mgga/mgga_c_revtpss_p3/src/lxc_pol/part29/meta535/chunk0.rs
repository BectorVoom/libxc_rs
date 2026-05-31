//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1867/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1867<F: Float>(t26474: F, t686: F, t72: F, t7058: F, t7064: F, t25387: F, t95571: F, t11050: F, t26497: F, t92975: F, t92988: F, t92995: F) -> (F, F, F, F, F, F, F) {
    let t95644 = t26474 * t72 * t686;
    let t95645 = t7058 * t95644;
    let t95647 = t7064 * t95644;
    let t95649 = t25387 * t95571;
    let t95651 = t26497 * t11050;
    let t95666 = F::cast_from(0.18295201011342718161e-3_f64) * t92975;
    let t95671 = F::cast_from(0.3252886739816735289e-3_f64) * t92988;
    let t95673 = F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t92995;
    (t95645, t95647, t95649, t95651, t95666, t95671, t95673)
}
