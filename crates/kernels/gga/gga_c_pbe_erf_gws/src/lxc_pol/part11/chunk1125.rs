//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1125/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1125<F: Float>(t32019: F, t3513: F, t12440: F, t7527: F, t12444: F, t1044: F, t1620: F, t1621: F, t40687: F, t18280: F, t47902: F, t47904: F, t47906: F, t47910: F, t47914: F, t47916: F) -> (F, F, F, F, F) {
    let t47918 = F::new(16.0) / F::new(5.0) * t32019 * t3513;
    let t47920 = F::new(16.0) / F::new(5.0) * t7527 * t12440;
    let t47922 = F::new(16.0) / F::new(5.0) * t7527 * t12444;
    let t47926 = F::new(16.0) / F::new(15.0) * t1620 * t1621 * t40687 * t1044;
    let t47927 = -t47902 - t47904 - t47906 + t18280 - t47910 + t47914 + t47916 - t47918 - t47920 - t47922 - t47926;
    (t47918, t47920, t47922, t47926, t47927)
}
