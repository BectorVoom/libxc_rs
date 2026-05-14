//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 440/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk440<F: Float>(t1943: F, t920: F, t1017: F, t72: F, t1023: F, t1526: F, t1527: F, t1942: F, t342: F, t343: F, t1537: F, t2: F, t4: F, t26: F) -> (F, F, F, F, F) {
    let t4641 = t1943 * t920;
    let t4645 = t72 * t1017;
    let t4649 = t1023 - t1942 - t1526 * t1527 * t4641 / 12.0 - t342 * t343 * t4645 / 4.0;
    let t5493 = t1537 * t2;
    let t5494 = t5493 * t4;
    let t5495 = t5494 * t26;
    (t4641, t4645, t4649, t5494, t5495)
}
