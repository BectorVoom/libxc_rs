//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1166/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1166<F: Float>(t100026: F, t100029: F, t100031: F, t100033: F, t100034: F, t100903: F, t100927: F, t101615: F, t101735: F, t187: F, t99837: F, t99839: F, t99842: F, t99845: F, t99847: F, t99850: F, t99852: F, t99854: F, t99856: F, t99859: F, t99861: F, t99864: F) -> (F,) {
    let t101739 = -t99837 + t99839 + t99842 + t99845 + t99847 + t99850 + t99852 + t99854 + t99856 + t99859 + t99861 - t99864 - t100026 - t100029 - t100031 + t100033 - t100034 + t187 * (t100903 + t100927 + t101615 + t101735);
    (t101739,)
}
