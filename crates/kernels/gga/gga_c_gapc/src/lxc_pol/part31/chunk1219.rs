//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1219/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1219<F: Float>(t34036: F, t36800: F, t36801: F, t36802: F, t36803: F, t36804: F, t36805: F, t36806: F, t36807: F, t36808: F, t36809: F, t34092: F, t34100: F, t36812: F, t36813: F, t36814: F, t36815: F, t36816: F, t36817: F, t36818: F, t36820: F, t36821: F) -> (F, F) {
    let t38788 = -0.11666621455439814815e-3 * t34036 + t36800 - t36801 - t36802 + t36803 - t36804 + t36805 + t36806 + t36807 + t36808 + t36809;
    let t38792 = -t36812 - t36813 - t36814 - t36815 - t36816 + t36817 + t36818 - 0.98380106748709416171e-8 * t34092 - t36820 + t36821 - 0.36231816839129402172e-6 * t34100;
    (t38788, t38792)
}
