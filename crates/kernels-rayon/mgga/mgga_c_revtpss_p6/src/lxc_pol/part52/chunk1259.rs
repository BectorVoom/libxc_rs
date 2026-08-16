//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1259/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1259(t102622: f64, t121059: f64, t121102: f64, t121109: f64, t121112: f64, t121118: f64, t122358: f64, t122455: f64, t125671: f64, t125677: f64, t125706: f64, t32719: f64, t34212: f64, t7925: f64) -> f64 {
    let t128688 = 0.25702851531048074406e-1_f64 * t122358 - 0.29749863367240808656e-2_f64 * t125671 + 0.225875734067843736e-2_f64 * t125677 + t121059 - t121102 - 0.34708173928447610099e-2_f64 * t125706 - t121109 + t121112 + t121118 - 0.11423947533020470523e1_f64 * t122455 * t34212 - 0.11423947533020470523e1_f64 * t32719 * t102622 * t7925;
    t128688
}
