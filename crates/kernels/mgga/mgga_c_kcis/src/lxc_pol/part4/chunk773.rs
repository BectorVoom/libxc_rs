//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 773/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk773<F: Float>(t5280: F, t5357: F, t1281: F, t1864: F, t1291: F, t1872: F, t5044: F, t5049: F, t5051: F, t5054: F, t5056: F, t5058: F, t5060: F, t5063: F, t5065: F, t5069: F, t5071: F, t5074: F, t5079: F) -> (F, F, F, F) {
    let t5358 = t5280 + t5357;
    let t5360 = t1864 * t1281;
    let t5363 = t1872 * t1291;
    let t5379 = -0.9375e-1 * t5044 + 0.1875e0 * t5049 - 0.13489583333333333333e-1 * t5051 + 0.25e0 * t5054 - 0.25e0 * t5056 + 0.625e-1 * t5058 + 0.625e-1 * t5060 - 0.625e-1 * t5063 - 0.13489583333333333333e-1 * t5065 + 0.101171875e-1 * t5069 - 0.9375e-1 * t5071 + 0.13489583333333333333e-1 * t5074 - 0.20833333333333333333e-1 * t5079;
    (t5358, t5360, t5363, t5379)
}
