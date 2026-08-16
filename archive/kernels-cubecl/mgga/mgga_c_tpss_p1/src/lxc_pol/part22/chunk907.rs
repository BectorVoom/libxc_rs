//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 907/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk907<F: Float>(t2149: F, t8167: F, t159: F, t799: F, t210: F, t2139: F, t760: F, t764: F, t2143: F, t2153: F, t64: F, t7091: F) -> (F, F, F, F, F, F) {
    let t8168 = t8167 * t2149;
    let t8170 = t159 * t799;
    let t8171 = t210 * t8170;
    let t8176 = t2139 * t760;
    let t8177 = t8176 * t764;
    let t8179 = t2143 * t2153;
    let t8185 = t64 * t7091;
    (t8168, t8171, t8176, t8177, t8179, t8185)
}
