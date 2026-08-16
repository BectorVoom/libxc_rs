//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2148/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2148(t19785: f64, t25517: f64, t100132: f64, t16509: f64, t16584: f64, t19622: f64, t19636: f64, t19726: f64, t19778: f64, t19782: f64, t20079: f64, t27492: f64, t27493: f64, t4896: f64, t4902: f64, t6268: f64, t93597: f64, t93658: f64, t93667: f64) -> f64 {
    let t106960 = t25517 * t19785;
    let t106968 = -0.30488190661738479625e-2_f64 * t93597 * t6268 + 0.17149607247227894789e-2_f64 * t93667 * t19622 + 0.57165357490759649296e-3_f64 * t27493 * t19726 + 0.57165357490759649296e-3_f64 * t25517 * t19778 + 0.28582678745379824648e-3_f64 * t25517 * t20079 - 0.17149607247227894789e-2_f64 * t93658 * t19636 + 0.47637797908966374413e-3_f64 * t25517 * t19782 + 0.38110238327173099531e-3_f64 * t106960 + t100132 + 0.17149607247227894789e-2_f64 * t16509 * t27492 * t4896 - 0.85748036236139473944e-3_f64 * t16584 * t27492 * t4902;
    t106968
}
