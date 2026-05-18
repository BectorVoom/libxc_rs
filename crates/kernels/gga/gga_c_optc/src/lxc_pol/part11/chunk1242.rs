//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1242/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1242<F: Float>(t3360: F, t48000: F, t10004: F, t2124: F, t2126: F, t2168: F, t3491: F, t3519: F, t37853: F, t48862: F, t48866: F, t48875: F, t48904: F, t56078: F, t56124: F, t56158: F, t56166: F, t56197: F, t56205: F, t56213: F, t56232: F, t686: F, t695: F, t696: F, t705: F, t7129: F) -> (F, F) {
    let t56501 = t48000 * t3360;
    let t56521 = -F::new(0.18137053605011111023e1) * t2168 * t56124 + F::new(0.69545291918310062836e0) * t2124 * t2126 * t56158 - F::new(0.18137053605011111023e1) * t2168 * t56078 + F::new(0.16227234780939014661e2) * t37853 + F::new(0.14604511302845113196e2) * t48862 + F::new(0.24340852171408521992e1) * t48866 + F::new(0.8463958349005185144e0) * t48875 - F::new(0.62590762726479056552e1) * t2124 * t7129 * t56501 - F::new(0.15114211337509259186e-1) * t695 * t696 * t56166 - F::new(0.45342634012527777558e-1) * t695 * t696 * t56205 - F::new(0.1251815254529581131e2) * t686 * t10004 * t56213 + F::new(0.20863587575493018851e1) * t686 * t3491 * t56232 + F::new(0.60456845350037036744e0) * t705 * t3519 * t56197 + F::new(0.33855833396020740576e1) * t48904;
    (t56501, t56521)
}
