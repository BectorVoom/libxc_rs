//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 894/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk894<F: Float>(t16954: F, t5309: F, t7136: F, t1898: F, t5304: F, t2704: F, t628: F, t1243: F, t1703: F, t1693: F, t395: F, t5093: F) -> (F, F, F, F, F, F, F) {
    let t16955 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t16954;
    let t16957 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t7136 * t5309;
    let t16959 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t5304 * t1898;
    let t16960 = t2704 * t628;
    let t16962 = t1243 * t1703;
    let t16964 = t1243 * t1693;
    let t16966 = t395 * t5093;
    (t16955, t16957, t16959, t16960, t16962, t16964, t16966)
}
