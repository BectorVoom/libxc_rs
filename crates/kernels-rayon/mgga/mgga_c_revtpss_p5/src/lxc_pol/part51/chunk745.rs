//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 745/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk745(t1061: f64, t72: f64, t3117: f64, t8502: f64, t8504: f64, t8508: f64, t8509: f64, t8514: f64, t8517: f64, t8522: f64, t1989: f64, t207: f64, t8489: f64) -> (f64, f64, f64, f64, f64) {
    let t8523 = t1061 * t72;
    let t8524 = t8523 * t3117;
    let t8527 = 0.28234466758480466999e-3_f64 * t8502 * t8504 - 0.8673628188205199462e0_f64 * t8508 * t8509 + 0.57119737665102352616e0_f64 * t8514 * t8517 - 0.1859366460452550541e-3_f64 * t8522 * t8524;
    let t8531 = t1989 * t1989;
    let t8536 = t207 * t8489;
    (t8523, t8524, t8527, t8531, t8536)
}
