//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 683/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk683<F: Float>(t462: F, t92: F, t9931: F, t9933: F, t9935: F, t9936: F, t9939: F, t9944: F, t9949: F, t9955: F, t9958: F, t9960: F, t9962: F, t9965: F, t9930: F, t734: F, t91: F) -> (F, F) {
    let t9967 = t462 * t9931 + t9933 - t9935 - 4.0 / 3.0 * t9936 - t462 * t9939 / 3.0 - 6.0 * t92 * t9944 + 6.0 * t462 * t9949 - 10.0 / 27.0 * t462 * t9955 + t9958 / 3.0 + 2.0 / 9.0 * t9960 - 2.0 * t9962 - t92 * t9965;
    let t9968 = t9930 + t9967;
    let t9970 = t91 * t734 * t9968;
    (t9968, t9970)
}
