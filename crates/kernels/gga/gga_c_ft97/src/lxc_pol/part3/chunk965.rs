//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 965/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk965<F: Float>(t18917: F, t898: F, t904: F, t16579: F, t231: F, t893: F, t10904: F, t5457: F, t17732: F, t4334: F, t668: F, t10864: F, t505: F) -> (F, F, F, F, F) {
    let t18919 = t898 * t18917 * t904;
    let t18923 = t231 * t893 * t16579;
    let t18926 = t10904 * t5457;
    let t18928 = t898 * t18926 * t904;
    let t18931 = t4334 * t17732;
    let t18934 = t5457 * t668;
    let t18936 = t10864 * t18934 * t505;
    (t18919, t18923, t18928, t18931, t18936)
}
