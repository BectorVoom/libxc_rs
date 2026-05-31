//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1122/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1122<F: Float>(t19191: F, t2380: F, t2383: F, t6475: F, t6484: F, t53: F, t6404: F, t179: F, t404: F, t6406: F, t414: F, t6545: F) -> (F, F, F, F) {
    let t19193 = t2380 * t19191 * t2383;
    let t19196 = t2380 * t6475 * t6484;
    let t19203 = t53 * t6404;
    let t19206 = t404 * t179 * t19203 * t6406;
    let t19227 = F::cast_from(1.0_f64) / t6545 / t414;
    (t19193, t19196, t19206, t19227)
}
