//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1124/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1124<F: Float>(t491: F, t6019: F, t1394: F, t7924: F, t28388: F, t98137: F, t28328: F, t4142: F, t7908: F, t98364: F, t15967: F, t28332: F, t28500: F, t27427: F, t28499: F, t27475: F, t303: F, t5633: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t98618 = t6019 * t491;
    let t98620 = t1394 * t98618 * t7924;
    let t98623 = 0.12378114784505208333e-4 * t28388 * t98137;
    let t98624 = t4142 * t28328;
    let t98625 = 0.22109259259259259258e-2 * t98624;
    let t98627 = 0.46336805555555555556e-3 * t7908 * t98364;
    let t98628 = t7908 * t98137;
    let t98632 = t15967 * t28332;
    let t98637 = t4142 * t28500;
    let t98640 = t1394 * t28499 * t27427;
    let t98643 = t303 * t27475 * t5633;
    (t98620, t98623, t98624, t98625, t98627, t98628, t98632, t98637, t98640, t98643)
}
