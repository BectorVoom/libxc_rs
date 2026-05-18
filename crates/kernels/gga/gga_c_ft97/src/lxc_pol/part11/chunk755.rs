//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 755/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk755<F: Float>(t2347: F, t294: F, t2349: F, t2360: F, t1934: F, t2639: F, t231: F, t2739: F, t10207: F, t10209: F, t10212: F, t1526: F, t2320: F, t2649: F, t2666: F, t2745: F, t342: F, t343: F, t3806: F) -> (F, F, F, F, F, F) {
    let t10214 = t294 * t2347;
    let t10215 = t10214 * t2349;
    let t10222 = t294 * t2360;
    let t10223 = t10222 * t2349;
    let t10227 = t2639 * t1934;
    let t10231 = t231 * t2739;
    let t10235 = t2649 + t2745 + t10207 - t10209 / F::new(18.0) - t10212 / F::new(6.0) - t1526 * t3806 * t10215 / F::new(9.0) - t1526 * t2320 * t2666 / F::new(6.0) + t1526 * t2320 * t10223 / F::new(6.0) - t1526 * t2320 * t10227 / F::new(12.0) - t342 * t343 * t10231 / F::new(4.0);
    (t10214, t10215, t10223, t10227, t10231, t10235)
}
