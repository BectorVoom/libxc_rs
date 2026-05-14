//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1098/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1098<F: Float>(t26954: F, t27013: F, t27069: F, t7772: F, t92751: F, t1250: F, t251: F, t34814: F, t92945: F, t35576: F, t1183: F, t982: F, t7771: F, t92794: F, t1014: F, t26811: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93023 = t27013 * t26954;
    let t93028 = t27069 * t26954;
    let t93047 = t7772 * t92751;
    let t93050 = t34814 * t251 * t1250;
    let t93053 = t7772 * t92945;
    let t93056 = t35576 * t251 * t1250;
    let t93059 = t1183 * t982;
    let t93082 = t7771 * t92794;
    let t93087 = t1014 * t26811;
    (t93023, t93028, t93047, t93050, t93053, t93056, t93059, t93082, t93087)
}
