//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1237/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1237<F: Float>(t15711: F, t7132: F, t16219: F, t7111: F, t11239: F, t1678: F, t1078: F, t1982: F, t3143: F, t7810: F, t3057: F, t25698: F, t27418: F) -> (F, F, F, F, F, F) {
    let t100343 = t7132 * t15711;
    let t100365 = t7111 * t16219;
    let t100533 = t1678 * t11239;
    let t100535 = t1982 * t100533 * t1078;
    let t100567 = t3143 * t7810;
    let t100681 = t3057 * t7810;
    let t100705 = t25698 * t27418;
    (t100343, t100365, t100535, t100567, t100681, t100705)
}
