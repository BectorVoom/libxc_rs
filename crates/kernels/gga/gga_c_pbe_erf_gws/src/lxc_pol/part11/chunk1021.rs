//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1021/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1021<F: Float>(t43: F, t50: F, t22063: F, t42442: F, t22066: F, t22068: F, t22070: F, t12345: F, t1402: F, t18670: F, t2457: F, t3346: F, t47: F, t47391: F, t47400: F, t47409: F, t9981: F, t12355: F, t1412: F, t18684: F, t2465: F, t3354: F, t47372: F, t47377: F, t47733: F, t52: F, t9993: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t48442 = 240.0 * t22063;
    let t48443 = 0.73246220147012639764e-3 * t42442;
    let t48444 = 0.65061485296689145286e-1 * t22066;
    let t48445 = 0.1926377843805564792e1 * t22068;
    let t48446 = 0.86748647062252193714e-1 * t22070;
    let t48458 = piecewise3(t44, 0.0, 40.0 / 81.0 * t18670 * t47391 - 16.0 / 9.0 * t9981 * t3346 + 4.0 / 3.0 * t1402 * t47409 + 16.0 / 9.0 * t2457 * t12345 + 4.0 / 3.0 * t47 * t47400);
    let t48470 = piecewise3(t51, 0.0, 40.0 / 81.0 * t18684 * t47377 - 16.0 / 9.0 * t9993 * t3354 + 4.0 / 3.0 * t1412 * t47733 + 16.0 / 9.0 * t2465 * t12355 + 4.0 / 3.0 * t52 * t47372);
    (t48442, t48443, t48444, t48445, t48446, t48458, t48470)
}
