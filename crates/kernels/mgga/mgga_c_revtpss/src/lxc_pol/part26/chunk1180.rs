//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1180/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1180<F: Float>(t93173: F, t95725: F, t93371: F, t26488: F, t686: F, t72: F, t93317: F, t26492: F, t25387: F, t93281: F, t2453: F, t26496: F) -> (F, F, F, F, F, F, F) {
    let t95746 = t95725 * t93173;
    let t95747 = t93371 * t95746;
    let t95761 = t26488 * t72 * t686;
    let t95762 = t93317 * t95761;
    let t95765 = t26492 * t72 * t686;
    let t95766 = t25387 * t95765;
    let t95768 = t93281 * t95761;
    let t95773 = t2453 * t26496;
    (t95746, t95747, t95762, t95765, t95766, t95768, t95773)
}
