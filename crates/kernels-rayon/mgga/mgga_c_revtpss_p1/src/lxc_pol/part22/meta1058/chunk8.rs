//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3761/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3761(t21107: f64, t3704: f64, t17628: f64, t5373: f64, t16750: f64, t1794: f64, t1042: f64, t1250: f64, t12976: f64, t16746: f64, t17237: f64, t17351: f64, t17426: f64, t17569: f64, t17589: f64, t20952: f64, t21085: f64, t21111: f64, t3647: f64, t3667: f64, t3711: f64, t3718: f64, t3720: f64, t5047: f64, t5277: f64, t5333: f64, t5391: f64, t59233: f64, t59239: f64, t6647: f64, t71245: f64) -> (f64, f64) {
    let t71710 = t21107 * t3704;
    let t71718 = t5373 * t17628;
    let t71724 = t16750 * t1794;
    let t71737 = 0.57165357490759649296e-3_f64 * t17569 * t17589 - 0.1270341277572436651e-2_f64 * t3647 * t21111 - 0.30488190661738479624e-2_f64 * t71710 + 0.28582678745379824648e-3_f64 * t3711 * t1042 * t5277 * t16746 + 0.67751534803863288053e-2_f64 * t5391 * t17237 - t71718 / 243.0_f64 + 0.1270341277572436651e-3_f64 * t59233 + 0.15244095330869239812e-2_f64 * t59239 + 0.17149607247227894789e-2_f64 * t17426 * t20952 - 0.42874018118069736972e-3_f64 * t3718 * t3720 * t71724 * t1250 - 0.95275595817932748826e-3_f64 * t17351 * t71245 * t5333 * t5047 - 0.21437009059034868486e-3_f64 * t12976 * t6647 - 0.42874018118069736972e-3_f64 * t3667 * t21085;
    (t71724, t71737)
}
