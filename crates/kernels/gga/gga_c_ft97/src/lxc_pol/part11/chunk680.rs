//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 680/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk680<F: Float>(t9745: F, t9916: F, t2: F, t9577: F, t9571: F, t2486: F, t3910: F, t9583: F, t2493: F, t9757: F, t462: F, t9897: F, t9900: F, t9903: F, t9905: F, t9907: F, t9910: F, t9913: F) -> (F, F, F, F, F, F, F) {
    let t9917 = t9916 * t9745;
    let t9920 = t2 * t9577;
    let t9921 = t9920 * t9571;
    let t9922 = t2486 * t9921;
    let t9925 = t3910 * t9583;
    let t9928 = t2493 * t9757;
    let t9930 = -2.0 * t462 * t9897 - 2.0 * t462 * t9900 - 2.0 / 3.0 * t9903 - 2.0 / 3.0 * t9905 - 4.0 / 9.0 * t9907 - 2.0 * t462 * t9910 + 2.0 * t462 * t9913 + 2.0 / 3.0 * t462 * t9917 + 4.0 / 3.0 * t462 * t9922 - 2.0 / 3.0 * t462 * t9925 + t462 * t9928;
    (t9917, t9920, t9921, t9922, t9925, t9928, t9930)
}
