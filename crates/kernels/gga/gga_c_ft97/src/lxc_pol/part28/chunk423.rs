//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 423/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk423<F: Float>(t103: F, t6524: F, t82: F, t5710: F, t979: F, t83: F, t5717: F, t925: F, t1909: F, t1332: F, t942: F) -> (F, F, F, F, F) {
    let t6526 = t82 * t6524 * t103;
    let t6530 = t5710 * t979;
    let t6531 = t83 * t6530;
    let t6534 = t5717 * t925;
    let t6535 = t1909 * t6534;
    let t6538 = t1332 * t942;
    (t6526, t6531, t6534, t6535, t6538)
}
