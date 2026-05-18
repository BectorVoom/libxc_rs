//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1262/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1262<F: Float>(t491: F, t6019: F, t1394: F, t7924: F, t28388: F, t98137: F, t28328: F, t4142: F, t7908: F, t98364: F, t15967: F, t28332: F) -> (F, F, F, F, F, F, F) {
    let t98618 = t6019 * t491;
    let t98620 = t1394 * t98618 * t7924;
    let t98623 = F::new(0.12378114784505208333e-4) * t28388 * t98137;
    let t98624 = t4142 * t28328;
    let t98625 = F::new(0.22109259259259259258e-2) * t98624;
    let t98627 = F::new(0.46336805555555555556e-3) * t7908 * t98364;
    let t98628 = t7908 * t98137;
    let t98632 = t15967 * t28332;
    (t98620, t98623, t98624, t98625, t98627, t98628, t98632)
}
