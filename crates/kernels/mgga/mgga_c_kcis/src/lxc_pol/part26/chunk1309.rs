//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1309/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1309<F: Float>(t21846: F, t4160: F, t94425: F, t17730: F, t1889: F, t6159: F, t11814: F, t29274: F, t1394: F, t8164: F, t98618: F, t28331: F, t28499: F, t5780: F) -> (F, F, F, F, F) {
    let t102431 = t4160 * t94425 * t21846;
    let t102438 = t6159 * t17730 * t1889;
    let t102441 = t11814 * t29274;
    let t102444 = t1394 * t98618 * t8164;
    let t102447 = t5780 * t28499 * t28331;
    (t102431, t102438, t102441, t102444, t102447)
}
