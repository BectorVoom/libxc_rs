//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 949/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk949<F: Float>(t738: F, t745: F, t9385: F, t1340: F, t1320: F, t3853: F, t123: F, t147: F, t9291: F) -> (F, F, F, F) {
    let t9387 = t738 * t9385 * t745;
    let t9389 = F::new(0.5848223622634646207e0) * t1340 * t9387;
    let t9391 = F::new(12.0) * t1320 * t3853;
    let t9394 = F::new(0.34450798614814814813e-2) * t123 * t9291 * t147;
    (t9387, t9389, t9391, t9394)
}
