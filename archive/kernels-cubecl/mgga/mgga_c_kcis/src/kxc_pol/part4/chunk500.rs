//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 500/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk500<F: Float>(t20: F, t2314: F, t21: F, t6: F, t736: F, t649: F, t66: F, t648: F, t119: F, t5: F) -> (F, F, F, F, F, F, F) {
    let t2315 = t2314 * t20;
    let t2316 = t21 * t6;
    let t2317 = t2316 * t736;
    let t2318 = t2315 * t2317;
    let t2320 = t649 * t66;
    let t2321 = t648 * t2320;
    let t2323 = t5 * t119;
    (t2315, t2316, t2317, t2318, t2320, t2321, t2323)
}
