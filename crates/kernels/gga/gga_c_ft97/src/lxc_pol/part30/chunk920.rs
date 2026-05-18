//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 920/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk920<F: Float>(t1443: F, t676: F, t6907: F, t737: F, t24737: F, t53798: F, t1456: F, t9895: F, t6154: F, t7021: F, t880: F, t1253: F, t6260: F) -> (F, F, F, F, F, F, F) {
    let t110751 = t676 * t1443;
    let t110950 = t737 * t6907;
    let t111089 = t53798 * t24737;
    let t111330 = t9895 * t1456;
    let t111518 = t737 * t6154;
    let t111668 = t7021 * t880;
    let t111711 = t6260 * t1253;
    (t110751, t110950, t111089, t111330, t111518, t111668, t111711)
}
