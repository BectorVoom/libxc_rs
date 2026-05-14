//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 636/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk636<F: Float>(t1291: F, t1872: F, t5044: F, t5049: F, t5051: F, t5054: F, t5056: F, t5058: F, t5060: F, t5063: F, t5065: F, t5069: F, t5071: F, t5074: F, t5079: F, t5084: F, t5087: F, t5089: F, t5092: F, t5094: F, t5097: F, t5100: F, t5166: F, t5170: F, t5173: F, t5178: F, t5183: F, t5186: F) -> (F, F, F) {
    let t5363 = t1872 * t1291;
    let t5379 = -0.9375e-1 * t5044 + 0.1875e0 * t5049 - 0.13489583333333333333e-1 * t5051 + 0.25e0 * t5054 - 0.25e0 * t5056 + 0.625e-1 * t5058 + 0.625e-1 * t5060 - 0.625e-1 * t5063 - 0.13489583333333333333e-1 * t5065 + 0.101171875e-1 * t5069 - 0.9375e-1 * t5071 + 0.13489583333333333333e-1 * t5074 - 0.20833333333333333333e-1 * t5079;
    let t5393 = -0.9375e-1 * t5084 + 0.71944444444444444443e-1 * t5087 + 0.101171875e-1 * t5089 - 0.625e-1 * t5092 + 0.53958333333333333333e-1 * t5094 - 0.53958333333333333333e-1 * t5097 + 0.13489583333333333333e-1 * t5100 + 0.9375e-1 * t5166 - 0.101171875e-1 * t5170 + 0.101171875e-1 * t5173 - 0.20234375e-1 * t5178 - 0.44965277777777777777e-2 * t5183 - 0.16666666666666666667e0 * t5186;
    (t5363, t5379, t5393)
}
