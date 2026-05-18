//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1169/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1169<F: Float>(t35068: F, t8392: F, t1053: F, t2179: F, t32992: F, t49562: F, t7400: F, t2142: F, t34947: F, t1882: F, t35181: F, t35217: F) -> (F, F, F, F, F, F) {
    let t148922 = t8392 * t35068;
    let t148943 = t2179 * t32992 * t1053;
    let t148955 = t49562 * t7400;
    let t148960 = t2142 * t34947;
    let t148964 = t1882 * t35181;
    let t148966 = t1882 * t35217;
    (t148922, t148943, t148955, t148960, t148964, t148966)
}
