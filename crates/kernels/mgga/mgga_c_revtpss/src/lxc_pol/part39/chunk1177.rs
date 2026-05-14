//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1177/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1177<F: Float>(t300: F, t4682: F, t983: F, t3030: F, t4719: F, t3034: F, t11591: F, t1642: F, t11524: F, t4732: F, t981: F, t2989: F, t3336: F, t5019: F, t11108: F, t1699: F) -> (F, F, F, F, F, F, F, F) {
    let t15547 = t300 * t4682;
    let t15549 = 0.11696447245269292414e1 * t15547 * t983;
    let t15551 = 0.5848223622634646207e0 * t4719 * t3030;
    let t15553 = 0.17315859105681463759e2 * t4719 * t3034;
    let t15555 = 0.5848223622634646207e0 * t11591 * t1642;
    let t15556 = t4732 * t11524;
    let t15558 = 0.17315859105681463759e2 * t981 * t15556;
    let t15559 = t4732 * t2989;
    let t15561 = 0.35089341735807877242e1 * t981 * t15559;
    let t15562 = t5019 * t3336;
    let t15566 = t1699 * t11108;
    (t15549, t15551, t15553, t15555, t15558, t15561, t15562, t15566)
}
