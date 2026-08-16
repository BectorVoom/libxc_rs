//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 909/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk909(t1235: f64, t1247: f64, t1791: f64, t1797: f64, t3600: f64, t3610: f64, t3625: f64, t3671: f64, t3711: f64, t484: f64, t5254: f64, t5256: f64, t5266: f64, t5274: f64, t5293: f64, t5323: f64, t5327: f64, t6595: f64, t6598: f64, t6602: f64, t6611: f64, t6619: f64, t6625: f64, t6631: f64, t6635: f64, t6640: f64, t6647: f64) -> f64 {
    let t6651 = 0.72409452821628889107e-2_f64 * t6595 * t484 - 0.22866142996303859718e-2_f64 * t6598 * t484 + 0.21437009059034868486e-3_f64 * t6602 * t484 - 0.22866142996303859718e-2_f64 * t5293 * t1797 - 0.15244095330869239812e-2_f64 * t5254 + 0.28582678745379824648e-3_f64 * t5256 + 0.42874018118069736972e-3_f64 * t3671 * t6611 + 0.22866142996303859718e-2_f64 * t5323 * t1791 + 0.42874018118069736972e-3_f64 * t5274 * t1797 + 0.28582678745379824648e-3_f64 * t3711 * t6619 + 0.21437009059034868486e-3_f64 * t1247 * t6625 + 0.42874018118069736972e-3_f64 * t3600 * t6631 - 0.21437009059034868486e-3_f64 * t3610 * t6635 - 0.28582678745379824648e-3_f64 * t3625 * t6640 - 0.42874018118069736972e-3_f64 * t5327 * t1791 - 0.21437009059034868486e-3_f64 * t1235 * t6647 + 0.28582678745379824648e-3_f64 * t5266;
    t6651
}
