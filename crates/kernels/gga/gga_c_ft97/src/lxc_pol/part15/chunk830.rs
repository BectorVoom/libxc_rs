//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 830/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk830<F: Float>(t1248: F, t5225: F, t2862: F, t871: F, t1212: F, t5299: F, t319: F, t4246: F, t5330: F, t840: F, t5393: F, t15147: F, t1901: F, t19318: F, t19320: F, t19322: F, t19343: F, t19387: F, t19389: F, t22178: F, t22183: F, t22188: F, t446: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22194 = t5225 * t1248;
    let t22196 = t2862 * t871 * t22194;
    let t22199 = t1212 * t5299;
    let t22201 = t2862 * t319 * t22199;
    let t22205 = t840 * t4246 * t5330;
    let t22208 = t5299 * t1248;
    let t22210 = t840 * t871 * t22208;
    let t22212 = t1212 * t5393;
    let t22214 = t840 * t871 * t22212;
    let t22216 = -F::new(2.0) / F::new(3.0) * t19318 + F::new(2.0) / F::new(27.0) * t19320 + t19322 / F::new(9.0) - F::new(2.0) / F::new(3.0) * t1901 * t22178 + F::new(2.0) / F::new(9.0) * t19343 - F::new(2.0) / F::new(3.0) * t1901 * t22183 + F::new(2.0) / F::new(9.0) * t1901 * t22188 - F::new(4.0) / F::new(9.0) * t15147 - F::new(2.0) / F::new(9.0) * t19387 + F::new(2.0) / F::new(3.0) * t19389 - F::new(2.0) * t446 * t22196 + F::new(2.0) * t446 * t22201 + F::new(2.0) * t446 * t22205 + t446 * t22210 + t446 * t22214;
    (t22194, t22196, t22199, t22201, t22205, t22208, t22210, t22212, t22214, t22216)
}
