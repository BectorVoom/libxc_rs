//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 863/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk863<F: Float>(t452: F, t7211: F, t986: F, t110: F, t34482: F, t34514: F, t83: F, t7281: F, t942: F, t488: F, t34569: F, t34542: F) -> (F, F, F, F, F, F, F) {
    let t34758 = t452 * t986 * t7211;
    let t34762 = t452 * t110 * t34482;
    let t34765 = t83 * t34514;
    let t34768 = t7281 * t942;
    let t34770 = t452 * t488 * t34768;
    let t34773 = t83 * t34569;
    let t34776 = t83 * t34542;
    (t34758, t34762, t34765, t34768, t34770, t34773, t34776)
}
