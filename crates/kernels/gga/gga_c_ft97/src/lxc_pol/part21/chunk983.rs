//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 983/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk983<F: Float>(t23548: F, t4822: F, t9144: F, t4827: F, t13220: F, t12703: F, t30223: F, t5916: F, t144: F, t30131: F, t1391: F, t4714: F, t574: F, t2185: F, t4668: F, t1017: F, t6718: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30446 = t23548 * t4822;
    let t30447 = t9144 * t30446;
    let t30450 = t23548 * t4827;
    let t30451 = t13220 * t30450;
    let t30454 = t12703 * t30223;
    let t30457 = t5916 * t4827;
    let t30458 = t9144 * t30457;
    let t30461 = t144 * t30131;
    let t30465 = t574 * t1391 * t4714;
    let t30469 = t2185 * t1391 * t4668;
    let t30472 = t6718 * t1017;
    (t30446, t30447, t30450, t30451, t30454, t30457, t30458, t30461, t30465, t30469, t30472)
}
