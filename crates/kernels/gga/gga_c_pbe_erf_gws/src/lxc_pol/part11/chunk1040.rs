//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1040/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1040<F: Float>(t47888: F, t47890: F, t47892: F, t47893: F, t47895: F, t47896: F, t47898: F, t47899: F, t47902: F, t47904: F, t47906: F, t47910: F, t47914: F, t47916: F, t47918: F, t47920: F, t47922: F, t47926: F, t47928: F, t48043: F, t48044: F, t48045: F) -> (F, F) {
    let t48657 = -t47888 - t47890 - t47892 - t47893 - t47895 + t47896 - t47898 - t47899 - t47902 - t47904 - t47906;
    let t48659 = -t47910 + t47914 + t47916 - t47918 - t47920 - t47922 - t47926 + t47928 - t48043 - t48044 + t48045;
    (t48657, t48659)
}
