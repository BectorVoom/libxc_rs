//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1079/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1079(t24543: f64, t482: f64, t13063: f64, t1042: f64, t22700: f64, t344: f64, t1261: f64, t13062: f64, t17377: f64, t17529: f64, t17569: f64, t17572: f64, t1808: f64, t20784: f64, t20787: f64, t20789: f64, t21143: f64, t21272: f64, t24535: f64, t464: f64, t5274: f64, t5391: f64, t6619: f64, t6625: f64, t6631: f64, t6635: f64, t6673: f64) -> (f64, f64, f64) {
    let t24544 = t482 * t24543;
    let t24545 = t24544 * t13063;
    let t24546 = t1042 * t24545;
    let t24551 = t22700 * t344;
    let t24562 = -0.14481890564325777821e-1_f64 * t21272 * t1808 - 0.3811023832717309953e-2_f64 * t5391 * t6673 - 0.63517063878621832552e-3_f64 * t1261 * t24535 - 0.42874018118069736972e-3_f64 * t21143 * t1808 + 0.57165357490759649295e-3_f64 * t20784 - 0.42874018118069736972e-3_f64 * t20787 - 0.45732285992607719436e-2_f64 * t20789 + 0.21437009059034868486e-3_f64 * t13062 * t24546 + 0.85748036236139473944e-3_f64 * t17569 * t6619 - 77.0_f64 / 162.0_f64 * t24551 * t464 + 0.34299214494455789577e-2_f64 * t17529 * t6635 + 0.64311027177104605458e-3_f64 * t5274 * t6625 + 0.12862205435420921092e-2_f64 * t17572 * t6631 - 0.64311027177104605458e-3_f64 * t17377 * t6635;
    (t24544, t24546, t24562)
}
