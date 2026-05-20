//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3249/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3249<F: Float>(t10318: F, t10327: F, t10380: F, t10407: F, t13334: F, t13406: F, t13409: F, t13414: F, t1470: F, t1471: F, t1486: F, t2291: F, t2312: F, t4182: F, t4187: F, t4188: F, t4191: F, t606: F, t607: F, t641: F, t72: F, t85: F) -> F {
    let t60391 = -t13406 * t641 / F::new(4.0) - t4187 * t2291 * t85 / F::new(4.0) - t13409 * t641 / F::new(2.0) - t1470 * t10380 * t85 / F::new(12.0) - t13414 * t641 / F::new(4.0) - t607 * t13334 * t85 / F::new(4.0) - t4182 * t2312 / F::new(4.0) - t4188 * t2312 / F::new(4.0) - t4191 * t2312 / F::new(4.0) - t1471 * t10407 / F::new(12.0) - t606 * t1486 * t72 * t10318 / F::new(4.0) - t10327 * t1486 * t85 / F::new(12.0);
    t60391
}
