//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1271/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1271<F: Float>(t18280: F, t3531: F, t6556: F, t6552: F, t3362: F, t5825: F, t606: F, t3417: F, t141: F, t1121: F, t18281: F, t1145: F) -> (F, F, F, F, F, F, F) {
    let t20256 = -t18280;
    let t20261 = F::cast_from(0.17315859105681463759e2_f64) * t3531 * t6556;
    let t20263 = F::cast_from(0.5848223622634646207e0_f64) * t3531 * t6552;
    let t20265 = t3362 * t5825;
    let t20266 = t20265 * t606;
    let t20267 = t3417 * t20266;
    let t20268 = t141 * t20267;
    let t20272 = t1121 * t18281;
    let t20273 = t1145 * t20272;
    (t20256, t20261, t20263, t20266, t20268, t20272, t20273)
}
