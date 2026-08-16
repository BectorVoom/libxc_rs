//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 752/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk752<F: Float>(t2132: F, t8422: F, t2131: F, t157: F, t2331: F, t406: F, t2152: F, t1410: F, t609: F, t2122: F, t556: F, t2147: F) -> (F, F, F, F, F, F) {
    let t8423 = t2132 * t8422;
    let t8424 = t2131 * t8423;
    let t8427 = t2331 * t406 * t157;
    let t8428 = t2152 * t8427;
    let t8432 = t609 * t1410 * t157;
    let t8433 = t2152 * t8432;
    let t8436 = t2122 * t556;
    let t8437 = t2147 * t8436;
    (t8423, t8424, t8428, t8433, t8436, t8437)
}
