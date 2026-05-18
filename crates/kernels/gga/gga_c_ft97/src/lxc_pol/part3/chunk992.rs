//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 992/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk992<F: Float>(t19362: F, t2862: F, t319: F, t5299: F, t875: F, t840: F, t871: F, t15147: F, t1901: F, t19318: F, t19320: F, t19322: F, t19326: F, t19330: F, t19335: F, t19340: F, t19343: F, t19346: F, t19351: F, t19355: F, t19359: F, t446: F) -> F {
    let t19364 = t2862 * t319 * t19362;
    let t19367 = t5299 * t875;
    let t19369 = t840 * t871 * t19367;
    let t19372 = -F::new(2.0) / F::new(9.0) * t19318 + F::new(2.0) / F::new(81.0) * t19320 + t19322 / F::new(27.0) + F::new(2.0) / F::new(3.0) * t446 * t19326 - F::new(2.0) / F::new(3.0) * t446 * t19330 - t446 * t19335 / F::new(3.0) + t1901 * t19340 / F::new(9.0) + F::new(2.0) / F::new(27.0) * t19343 - F::new(2.0) / F::new(3.0) * t446 * t19346 - F::new(8.0) / F::new(27.0) * t15147 + F::new(2.0) / F::new(9.0) * t446 * t19351 + F::new(4.0) / F::new(3.0) * t446 * t19355 + F::new(2.0) / F::new(3.0) * t446 * t19359 + F::new(2.0) / F::new(3.0) * t446 * t19364 + t446 * t19369 / F::new(3.0);
    t19372
}
