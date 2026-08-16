//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2208/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2208(t15731: f64, t7122: f64, t15938: f64, t16017: f64, t16070: f64, t16144: f64, t16196: f64, t16210: f64, t1671: f64, t1675: f64, t25522: f64, t27498: f64, t4912: f64, t7132: f64, t93541: f64, t93561: f64, t93649: f64, t93670: f64, t99983: f64, t99985: f64) -> f64 {
    let t100002 = t7122 * t15731;
    let t100004 = 0.3811023832717309953e-3_f64 * t93541 + 0.28582678745379824648e-3_f64 * t93561 * t1675 + t99983 + 0.42874018118069736972e-3_f64 * t99985 * t16070 + 0.45732285992607719436e-2_f64 * t93670 * t4912 - 0.85748036236139473944e-3_f64 * t27498 * t16017 + 0.57165357490759649296e-3_f64 * t25522 * t16144 - 0.45732285992607719436e-2_f64 * t93649 * t1671 + 0.17149607247227894789e-2_f64 * t7132 * t15938 - 0.57165357490759649296e-3_f64 * t7132 * t16196 + 0.1270341277572436651e-2_f64 * t7132 * t16210 - 0.95275595817932748827e-4_f64 * t100002;
    t100004
}
