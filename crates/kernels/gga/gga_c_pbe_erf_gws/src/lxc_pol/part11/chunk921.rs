//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 921/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk921<F: Float>(t12891: F, t700: F, t12381: F, t153: F, t542: F, t13045: F, t168: F, t703: F, t13008: F, t256: F, t719: F, t12323: F, t19: F, t336: F, t714: F, t13039: F, t735: F) -> (F, F, F, F, F, F) {
    let t42923 = t12891 * t700;
    let t42928 = t153 * t542 * t12381;
    let t42935 = t168 * t703 * t13045;
    let t42943 = t13008 * t719 * t256;
    let t42948 = t12323 * t19 * t336 * t714;
    let t42953 = t13039 * t735;
    (t42923, t42928, t42935, t42943, t42948, t42953)
}
