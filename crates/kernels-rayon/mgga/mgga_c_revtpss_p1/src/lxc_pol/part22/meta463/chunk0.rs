//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2144/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2144(t15547: f64, t983: f64, t3030: f64, t4719: f64, t3034: f64, t11591: f64, t1642: f64, t11524: f64, t4732: f64, t981: f64, t2989: f64, t3336: f64, t5019: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15549 = 0.11696447245269292414e1_f64 * t15547 * t983;
    let t15551 = 0.5848223622634646207e0_f64 * t4719 * t3030;
    let t15553 = 0.17315859105681463759e2_f64 * t4719 * t3034;
    let t15555 = 0.5848223622634646207e0_f64 * t11591 * t1642;
    let t15556 = t4732 * t11524;
    let t15558 = 0.17315859105681463759e2_f64 * t981 * t15556;
    let t15559 = t4732 * t2989;
    let t15561 = 0.35089341735807877242e1_f64 * t981 * t15559;
    let t15562 = t5019 * t3336;
    (t15549, t15551, t15553, t15555, t15556, t15558, t15559, t15561, t15562)
}
