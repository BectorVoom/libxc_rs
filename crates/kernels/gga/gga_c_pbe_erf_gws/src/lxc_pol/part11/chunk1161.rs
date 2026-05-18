//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1161/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1161<F: Float>(t43: F, t21975: F, t30116: F, t22063: F, t42442: F, t22066: F, t22068: F, t22070: F, t12345: F, t1402: F, t18670: F, t2457: F, t3346: F, t47: F, t47391: F, t47400: F, t47409: F, t9981: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t48440 = F::new(384.0) * t21975;
    let t48441 = F::new(6.0) * t30116;
    let t48442 = F::new(240.0) * t22063;
    let t48443 = F::new(0.73246220147012639764e-3) * t42442;
    let t48444 = F::new(0.65061485296689145286e-1) * t22066;
    let t48445 = F::new(0.1926377843805564792e1) * t22068;
    let t48446 = F::new(0.86748647062252193714e-1) * t22070;
    let t48458 = piecewise3::<f64>(t44, F::new(0.0), F::new(40.0) / F::new(81.0) * t18670 * t47391 - F::new(16.0) / F::new(9.0) * t9981 * t3346 + F::new(4.0) / F::new(3.0) * t1402 * t47409 + F::new(16.0) / F::new(9.0) * t2457 * t12345 + F::new(4.0) / F::new(3.0) * t47 * t47400);
    (t48440, t48441, t48442, t48443, t48444, t48445, t48446, t48458)
}
