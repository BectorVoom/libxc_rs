//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 710/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk710<F: Float>(t2801: F, t684: F, t870: F, t2881: F, t2770: F, t863: F, t2877: F, t848: F, t2884: F, t2842: F, t2844: F, t2739: F, t312: F, t2874: F, t2878: F, t8392: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10439 = t870 * t2801 * t684;
    let t10440 = t2881 * t10439;
    let t10443 = t2770 * t863;
    let t10444 = t10443 * t2877;
    let t10447 = t848 * t863;
    let t10448 = t10447 * t2884;
    let t10452 = t2842 * t2844 * t684;
    let t10453 = t2881 * t10452;
    let t10457 = t312 * t2739 * t684;
    let t10458 = t2874 * t10457;
    let t10461 = t8392 * t2878;
    (t10439, t10440, t10443, t10444, t10447, t10448, t10452, t10453, t10457, t10458, t10461)
}
