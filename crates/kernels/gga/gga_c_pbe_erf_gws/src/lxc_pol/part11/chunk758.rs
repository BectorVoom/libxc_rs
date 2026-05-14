//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 758/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk758<F: Float>(t13126: F, t6801: F, t1115: F, t12130: F, t12138: F, t12182: F, t13096: F, t13105: F, t13112: F, t13121: F, t2503: F, t3047: F, t3052: F, t3917: F, t3921: F, t833: F, t8629: F, t8793: F, t9852: F, t9854: F, t9879: F, t9885: F, t9890: F, t9907: F) -> (F, F) {
    let t13127 = t13126 * t6801;
    let t13137 = -t1115 * t12138 / 8.0 - t1115 * t9890 / 16.0 + t12130 * t13096 / 32.0 - t3917 * t3047 / 32.0 - t3921 * t3047 / 32.0 + t9907 * t13105 / 32.0 - t1115 * t9885 / 16.0 + t8629 * t13112 / 16.0 - t3917 * t3052 / 16.0 + 7.0 / 48.0 * t9852 + 7.0 / 24.0 * t9854 + t8629 * t13121 / 32.0 + t13127 * t833 / 96.0 + t3917 * t2503 / 32.0 - 7.0 / 48.0 * t9879 + t8793 * t12182 / 8.0 - t3921 * t3052 / 16.0;
    (t13127, t13137)
}
