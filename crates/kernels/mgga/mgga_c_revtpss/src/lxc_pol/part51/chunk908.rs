//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 908/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk908<F: Float>(t116: F, t28159: F, t1892: F, t7063: F, t30: F, t41154: F, t1568: F, t1096: F, t357: F, t1695: F, t988: F, t1678: F, t7150: F, t1651: F, t11239: F, t1646: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t97622 = t28159 * t116;
    let t98040 = t7063 * t1892;
    let t98785 = t41154 * t30;
    let t98848 = t7063 * t1568;
    let t99566 = t357 * t1096;
    let t99638 = t1695 * t988;
    let t99914 = t7150 * t1678;
    let t99970 = t1651 * t988;
    let t100533 = t1678 * t11239;
    let t100743 = t1646 * t1096;
    (t97622, t98040, t98785, t98848, t99566, t99638, t99914, t99970, t100533, t100743)
}
