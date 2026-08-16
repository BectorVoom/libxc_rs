//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 887/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk887<F: Float>(t1704: F, t4621: F, t14546: F, t1003: F, t6330: F, t2894: F, t18570: F, t4947: F, t14554: F, t18574: F, t6334: F, t18677: F) -> (F, F, F, F, F, F) {
    let t19218 = t4621 * t1704;
    let t19219 = t14546 * t19218;
    let t19222 = t6330 * t1003;
    let t19223 = t2894 * t19222;
    let t19226 = t4947 * t18570;
    let t19229 = t14554 * t18574;
    let t19232 = t6334 * t1003;
    let t19233 = t2894 * t19232;
    let t19236 = t4947 * t18677;
    (t19219, t19223, t19226, t19229, t19233, t19236)
}
