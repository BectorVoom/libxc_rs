//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 906/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk906<F: Float>(t10596: F, t10597: F, t10598: F, t10600: F, t5028: F, t5040: F, t5066: F, t5069: F, t5073: F, t5324: F, t5333: F, t5338: F, t5344: F, t10589: F, t10590: F, t10595: F, t158: F) -> (F,) {
    let t10601 = t10596 + t5028 - t10597 - t10598 + t10600 - t5324 + t5040 + t5066 - t5069 - t5073 + t5333 - t5338 - t5344;
    let t10604 = (t10589 + t10590 + t10595 + t10601) * t158;
    (t10604,)
}
