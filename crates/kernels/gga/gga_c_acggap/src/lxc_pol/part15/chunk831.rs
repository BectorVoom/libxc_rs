//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 831/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk831<F: Float>(t2032: F, t31258: F, t2059: F, t7599: F, t2062: F, t167: F, t7309: F, t7483: F, t7310: F, t7487: F, t2082: F, t30044: F, t2087: F, t7610: F, t381: F, t7779: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31259 = t31258 * t2032;
    let t31261 = t7599 * t2059;
    let t31262 = t31261 * t2062;
    let t31276 = t7309 * t167;
    let t31277 = t31276 * t7483;
    let t31279 = t7310 * t7487;
    let t31283 = t30044 * t2082;
    let t31285 = t7610 * t2087;
    let t31289 = t381 * t7779;
    (t31259, t31261, t31262, t31276, t31277, t31279, t31283, t31285, t31289)
}
