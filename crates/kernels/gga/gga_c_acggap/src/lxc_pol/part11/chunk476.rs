//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 476/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk476<F: Float>(t2282: F, t578: F, t2041: F, t500: F, t1969: F, t1971: F, t1987: F, t1990: F, t1996: F, t2000: F, t2011: F, t2014: F, t2018: F, t2258: F, t2261: F, t2265: F, t2269: F, t2271: F, t2275: F, t2279: F) -> F {
    let t2283 = t578 * t2282;
    let t2285 = t2041 * t500;
    let t2287 = t1969 - t1971 + t1987 - t1990 - t1996 - t2000 - F::cast_from(0.17149607247227894789e-2_f64) * t2258 - t2011 + t2014 + t2261 / F::new(96.0) - F::cast_from(0.10718504529517434243e-3_f64) * t2265 + F::cast_from(0.15724046144802076034e-3_f64) * t2269 + t2018 - t2271 / F::new(96.0) - t2275 / F::new(128.0) - t2279 / F::new(384.0) - F::new(0.38203125e-2) * t2283 - t2285 / F::new(48.0);
    t2287
}
