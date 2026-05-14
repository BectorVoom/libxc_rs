//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 684/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk684<F: Float>(t2206: F, t3867: F, t2289: F, t3827: F, t3857: F, t3802: F, t4394: F, t3824: F, t6: F) -> (F, F, F, F, F) {
    let t11493 = t2206 * t3867;
    let t11495 = t2289 * t3827;
    let t11497 = t2289 * t3857;
    let t11499 = t3802 * t4394;
    let t11514 = t6 * t3824;
    (t11493, t11495, t11497, t11499, t11514)
}
