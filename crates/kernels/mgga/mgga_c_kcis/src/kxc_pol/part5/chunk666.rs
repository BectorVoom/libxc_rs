//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 666/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk666<F: Float>(t4813: F, t5077: F, t5076: F, t5044: F, t5049: F, t5051: F, t5054: F, t5056: F, t5058: F, t5060: F, t5063: F, t5065: F, t5069: F, t5071: F, t5074: F, t1094: F, t1795: F) -> (F, F, F, F) {
    let t5078 = t5077 * t4813;
    let t5079 = t5076 * t5078;
    let t5081 = -t5044 / 16.0 + t5049 / 8.0 - t5051 / 192.0 + t5054 / 6.0 - t5056 / 6.0 + t5058 / 24.0 + t5060 / 24.0 - t5063 / 24.0 - t5065 / 192.0 + t5069 / 256.0 - t5071 / 16.0 + t5074 / 192.0 - t5079 / 72.0;
    let t5082 = t1795 * t1094;
    (t5078, t5079, t5081, t5082)
}
