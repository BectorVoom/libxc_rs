//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1111/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1111<F: Float>(t1092: F, t13321: F, t27764: F, t283: F, t27792: F, t27836: F, t100319: F, t5302: F, t93089: F, t1800: F, t27763: F, t4772: F, t1020: F, t8047: F, t95893: F, t2822: F, t28908: F) -> (F, F, F, F, F, F) {
    let t100383 = t1092 * t13321 * t283 * t27764;
    let t100386 = t1092 * t27836 * t27792;
    let t100389 = t5302 * t93089 * t100319;
    let t100398 = t1092 * t27763 * t1800 * t4772;
    let t100401 = t1020 * t95893 * t8047;
    let t100407 = t2822 * t28908;
    (t100383, t100386, t100389, t100398, t100401, t100407)
}
