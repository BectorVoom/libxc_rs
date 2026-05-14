//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 925/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk925<F: Float>(t12234: F, t3083: F, t11348: F, t2503: F, t376: F, t3780: F, t13126: F, t4396: F, t20142: F, t833: F, t13680: F, t840: F, t13223: F, t21825: F, t13184: F, t8801: F) -> (F, F, F, F, F, F, F, F) {
    let t43788 = t3083 * t12234;
    let t43790 = t11348 * t2503;
    let t43814 = t376 * t3780;
    let t43872 = t13126 * t4396;
    let t43887 = t13126 * t20142 * t833;
    let t43889 = t840 * t13680;
    let t43903 = t21825 * t13223;
    let t43917 = t8801 * t13184;
    (t43788, t43790, t43814, t43872, t43887, t43889, t43903, t43917)
}
