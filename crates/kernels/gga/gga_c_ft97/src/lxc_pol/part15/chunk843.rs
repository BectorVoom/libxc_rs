//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 843/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk843<F: Float>(t1212: F, t4635: F, t2875: F, t2874: F, t1248: F, t2882: F, t2881: F, t15318: F, t1901: F, t19635: F, t22261: F, t22348: F, t22352: F, t22357: F, t22361: F, t22364: F, t22369: F, t22373: F, t22377: F, t22380: F, t22383: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t22386 = t4635 * t1212;
    let t22387 = t2875 * t22386;
    let t22388 = t2874 * t22387;
    let t22391 = t4635 * t1248;
    let t22392 = t2882 * t22391;
    let t22393 = t2881 * t22392;
    let t22396 = F::new(2.0) / F::new(3.0) * t446 * t22261 - t446 * t22348 / F::new(3.0) - F::new(2.0) * t446 * t22352 - t19635 / F::new(3.0) + F::new(2.0) * t446 * t22357 - t446 * t22361 + F::new(2.0) * t446 * t22364 - F::new(4.0) / F::new(27.0) * t15318 + F::new(2.0) / F::new(9.0) * t1901 * t22369 - F::new(2.0) / F::new(9.0) * t1901 * t22373 - F::new(2.0) / F::new(3.0) * t1901 * t22377 + F::new(2.0) / F::new(3.0) * t1901 * t22380 + F::new(2.0) / F::new(3.0) * t1901 * t22383 + t1901 * t22388 / F::new(3.0) + t1901 * t22393 / F::new(3.0);
    (t22386, t22387, t22388, t22391, t22392, t22393, t22396)
}
