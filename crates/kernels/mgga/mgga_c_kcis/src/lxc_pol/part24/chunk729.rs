//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 729/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk729<F: Float>(t414: F, t982: F, t990: F, t209: F, t287: F, t421: F, t736: F, t416: F, t1242: F, t3497: F, t1236: F, t3643: F, t1238: F, t413: F, t10471: F, t1278: F, t3668: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11086 = t414 * t982 * t990;
    let t11091 = t209 * t736 * t287 * t421;
    let t11093 = 5.0 / 2592.0 * t416 * t11091;
    let t11100 = t1242 * t3497;
    let t11151 = t1236 * t3643;
    let t11181 = t1238 * t1238;
    let t11182 = 1.0 / t11181;
    let t11183 = t413 * t11182;
    let t11209 = 0.51588271604938271604e-3 * t10471;
    let t11223 = t1278 * t3668;
    (t11086, t11093, t11100, t11151, t11181, t11182, t11183, t11209, t11223)
}
