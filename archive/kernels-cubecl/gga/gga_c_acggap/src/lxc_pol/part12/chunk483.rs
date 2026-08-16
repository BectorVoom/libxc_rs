//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 483/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk483<F: Float>(t2310: F, t598: F, t495: F, t599: F, t142: F, t2030: F, t513: F, t604: F, t2060: F, t2001: F, t537: F, t542: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2311 = t598 * t2310;
    let t2313 = t599 * t495;
    let t2314 = t142 * t2313;
    let t2315 = t2030 * t2314;
    let t2317 = t604 * t513;
    let t2318 = t142 * t2317;
    let t2319 = t2060 * t2318;
    let t2321 = t2001 * t537;
    let t2323 = t2001 * t542;
    (t2311, t2313, t2314, t2315, t2317, t2318, t2319, t2321, t2323)
}
