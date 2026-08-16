//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3750/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3750(t17693: f64, t20937: f64, t56756: f64, t1222: f64, t17240: f64, t20310: f64, t12832: f64, t12866: f64, t17170: f64, t17351: f64, t17353: f64, t17420: f64, t17513: f64, t17703: f64, t17705: f64, t20800: f64, t21049: f64, t21259: f64, t3603: f64, t3604: f64, t3611: f64, t3720: f64, t44510: f64, t44517: f64, t5332: f64, t5340: f64, t5401: f64, t59040: f64, t59043: f64, t59062: f64, t69839: f64, t70633: f64) -> f64 {
    let t71341 = t17693 * t56756 * t20937;
    let t71373 = t1222 * t17240 * t20310;
    let t71375 = 0.57165357490759649296e-3_f64 * t12866 * t59062 * t5401 - 0.76220476654346199061e-3_f64 * t71341 - 0.28582678745379824648e-3_f64 * t44517 * t69839 * t3611 * t17513 + 0.57165357490759649296e-3_f64 * t44510 * t69839 * t3604 * t17513 + 0.57165357490759649296e-3_f64 * t17351 * t17353 * t3611 * t70633 + 0.17149607247227894789e-2_f64 * t21049 * t17420 + 0.42874018118069736972e-3_f64 * t5340 * t3720 * t20800 * t17703 + 0.85748036236139473944e-3_f64 * t21049 * t17705 - 0.10162730220579493208e-2_f64 * t59040 + 0.85748036236139473944e-3_f64 * t5340 * t3720 * t5332 * t3603 * t17170 - 0.85748036236139473944e-3_f64 * t12832 * t21259 - 0.57165357490759649296e-3_f64 * t59043 - t71373 / 108.0_f64;
    t71375
}
