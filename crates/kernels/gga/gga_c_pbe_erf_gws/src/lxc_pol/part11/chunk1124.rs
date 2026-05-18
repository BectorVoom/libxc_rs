//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1124/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1124<F: Float>(t32215: F, t3479: F, t3555: F, t1033: F, t12871: F, t1896: F, t47409: F, t587: F, t590: F, t1661: F, t1664: F, t10843: F, t3531: F) -> (F, F, F, F, F, F) {
    let t47902 = F::new(16.0) / F::new(45.0) * t32215;
    let t47904 = F::new(4.0) / F::new(5.0) * t3479 * t3555;
    let t47906 = F::new(8.0) / F::new(15.0) * t1033 * t12871;
    let t47910 = F::new(8.0) / F::new(15.0) * t587 * t590 * t1896 * t47409;
    let t47914 = F::new(4.0) / F::new(9.0) * t587 * t1661 * t1664 * t47409;
    let t47916 = F::new(16.0) / F::new(9.0) * t10843 * t3531;
    (t47902, t47904, t47906, t47910, t47914, t47916)
}
