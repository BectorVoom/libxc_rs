//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 563/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk563<F: Float>(t821: F, t823: F, t198: F, t207: F, t2208: F, t2217: F, t2220: F, t2242: F, t2244: F, t2246: F, t2292: F, t2302: F, t2310: F, t2333: F, t2347: F, t2433: F, t2436: F, t2439: F, t750: F) -> (F, F) {
    let t2440 = t821 * t823;
    let t2444 = -t198 * t207 * t2433 * t2436 + F::cast_from(6.0_f64) * t2439 * t2440 * t750 - t2208 - t2217 - t2220 + t2242 + t2244 + t2246 - t2292 + t2302 + t2310 + t2333 + t2347;
    (t2440, t2444)
}
