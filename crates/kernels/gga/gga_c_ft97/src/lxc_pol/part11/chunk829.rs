//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 829/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk829<F: Float>(t1354: F, t1995: F, t23809: F, t527: F, t7240: F, t81: F, t1693: F, t395: F, t142: F, t7367: F, t240: F, t7513: F) -> (F, F, F, F, F, F, F) {
    let t23847 = t1995 * t1354;
    let t23866 = t1995 * t23809;
    let t23869 = t527 * t1354;
    let t32075 = F::new(1.0) / t7240 / t81;
    let t32211 = t1693 * t395;
    let t32905 = F::new(1.0) / t7367 / t142;
    let t33300 = F::new(1.0) / t7513 / t240;
    (t23847, t23866, t23869, t32075, t32211, t32905, t33300)
}
