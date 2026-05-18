//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 679/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk679<F: Float>(t301: F, t7381: F, t7380: F, t1983: F, t372: F, t2095: F, t355: F, t429: F, t2016: F, t2056: F, t1981: F, t576: F) -> (F, F, F, F, F, F, F, F) {
    let t7382 = t7381 * t301;
    let t7383 = t7380 * t7382;
    let t7386 = t1983 * t372;
    let t7387 = t2095 * t7386;
    let t7389 = t429 * t355;
    let t7390 = t2095 * t7389;
    let t7396 = t2016 * t2056;
    let t7400 = t576 * t1981;
    (t7382, t7383, t7386, t7387, t7389, t7390, t7396, t7400)
}
