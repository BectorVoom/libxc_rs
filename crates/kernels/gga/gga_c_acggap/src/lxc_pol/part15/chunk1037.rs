//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1037/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1037<F: Float>(t30159: F, t36213: F, t7586: F, t2299: F, t7630: F, t1413: F, t7712: F, t2310: F, t31849: F, t30248: F, t542: F, t1967: F, t8855: F) -> (F, F, F, F, F, F, F) {
    let t36302 = t30159 * t7586 * t36213;
    let t36327 = t7630 * t2299;
    let t36331 = t7712 * t1413;
    let t36333 = t7630 * t2310;
    let t36340 = F::new(0.15724046144802076034e-2) * t31849;
    let t36349 = t30248 * t542;
    let t36351 = t1967 * t8855;
    (t36302, t36327, t36331, t36333, t36340, t36349, t36351)
}
