//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 860/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk860<F: Float>(t13598: F, t1526: F, t21103: F, t4922: F, t9483: F, t21114: F, t21110: F, t21118: F, t342: F, t630: F, t22479: F, t5: F, t21843: F, t2253: F, t21847: F, t21837: F, t2938: F) -> (F, F, F, F, F, F, F, F, F) {
    let t81955 = t1526 * t13598 * t21103;
    let t81958 = t1526 * t9483 * t4922;
    let t81968 = t1526 * t9483 * t21114;
    let t81971 = t1526 * t9483 * t21110;
    let t81974 = t342 * t630 * t21118;
    let t82074 = t5 * t22479;
    let t82077 = t2253 * t21843;
    let t82079 = t2253 * t21847;
    let t82082 = t2938 * t21837;
    (t81955, t81958, t81968, t81971, t81974, t82074, t82077, t82079, t82082)
}
