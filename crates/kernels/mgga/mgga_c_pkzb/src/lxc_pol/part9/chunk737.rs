//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 737/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk737<F: Float>(t5342: F, t555: F, t5025: F, t5028: F, t5040: F, t5066: F, t5069: F, t5073: F, t5324: F, t5326: F, t5329: F, t5333: F, t5338: F, t5340: F, t158: F, t5317: F, t5318: F, t5320: F) -> (F, F) {
    let t5344 = 0.5848223622634646207e0 * t555 * t5342;
    let t5345 = t5025 + t5028 - t5324 + t5040 + t5066 - t5069 - t5073 + t5326 - t5329 + t5333 - t5338 - t5340 - t5344;
    let t5348 = (t5317 + t5318 + t5320 + t5345) * t158;
    (t5344, t5348)
}
