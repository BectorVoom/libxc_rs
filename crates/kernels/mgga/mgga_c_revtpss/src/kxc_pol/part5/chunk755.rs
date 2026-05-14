//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 755/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk755<F: Float>(t1248: F, t3603: F, t5332: F, t3720: F, t1774: F, t1250: F, t1794: F, t73: F) -> (F, F, F, F, F, F, F) {
    let t5341 = t3603 * t1248;
    let t5342 = t5332 * t5341;
    let t5343 = t3720 * t5342;
    let t5346 = t1774 * t1248;
    let t5347 = t5346 * t1250;
    let t5348 = t3720 * t5347;
    let t5351 = t1794 * t73;
    (t5341, t5342, t5343, t5346, t5347, t5348, t5351)
}
