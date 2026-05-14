//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 392/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk392<F: Float>(t1917: F, t24: F, t712: F, t1243: F, t1251: F, t248: F, t256: F, t528: F, t713: F, t1365: F, t153: F, t274: F, t164: F, t762: F, t547: F, t147: F, t837: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1918 = t24 * t1917;
    let t1920 = 0.12155555555555555555e0 * t712 * t1918;
    let t1923 = -0.43111111111111111111e-1 * t1243 + 0.18777777777777777778e0 * t1251;
    let t1924 = t248 * t1923;
    let t1926 = t1924 * t256 / 3.0;
    let t1928 = 0.33245444444444444444e-1 * t528 * t713;
    let t1937 = 0.13287210228946179141e1 * t153 * t1365 * t274;
    let t1947 = 0.63010814446282235668e-1 * t762 * t164;
    let t1951 = 0.63010814446282235668e-1 * t528 * t547;
    let t1952 = t837 * t147;
    (t1918, t1920, t1923, t1924, t1926, t1928, t1937, t1947, t1951, t1952)
}
