//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1225/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1225<F: Float>(t32628: F, t32647: F, t111042: F, t9368: F, t32658: F, t32664: F, t111026: F, t111030: F, t111032: F, t111034: F, t111038: F, t111040: F, t111043: F, t111049: F, t111051: F, t111048: F, t9379: F) -> (F, F) {
    let t111053 = t32647 * t32628;
    let t111055 = t111042 * t9368;
    let t111057 = t32664 * t32658;
    let t111059 = -0.72916666666666666668e-1 * t111026 + 0.24305555555555555556e0 * t111030 + 0.24305555555555555556e0 * t111032 - 0.14583333333333333334e0 * t111034 - 0.14583333333333333334e0 * t111038 - 0.14583333333333333334e0 * t111040 + 0.31250000000000000001e-1 * t111043 + 0.17972642500000000001e-2 * t111049 - 0.62500000000000000002e-1 * t111051 - 0.62500000000000000002e-1 * t111053 + 0.31250000000000000001e-1 * t111055 + 0.31250000000000000001e-1 * t111057;
    let t111062 = t9379 * t111048;
    (t111059, t111062)
}
