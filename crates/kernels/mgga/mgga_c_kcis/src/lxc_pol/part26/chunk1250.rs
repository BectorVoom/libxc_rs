//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1250/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1250<F: Float>(t28328: F, t4142: F, t7908: F, t98364: F, t98137: F, t15967: F, t28332: F, t28500: F, t28423: F, t7895: F, t18210: F, t2237: F, t28534: F) -> (F, F, F, F, F, F, F, F) {
    let t98624 = t4142 * t28328;
    let t98625 = F::new(0.22109259259259259258e-2) * t98624;
    let t98627 = F::new(0.46336805555555555556e-3) * t7908 * t98364;
    let t98628 = t7908 * t98137;
    let t98632 = t15967 * t28332;
    let t98637 = t4142 * t28500;
    let t98649 = F::new(0.46336805555555555556e-3) * t7895 * t28423;
    let t98652 = F::new(0.46336805555555555556e-3) * t2237 * t18210 * t28534;
    (t98624, t98625, t98627, t98628, t98632, t98637, t98649, t98652)
}
