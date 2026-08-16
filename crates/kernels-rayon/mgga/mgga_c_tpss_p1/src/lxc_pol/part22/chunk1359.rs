//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1359/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1359(t1497: f64, t1692: f64, t18268: f64, t18728: f64, t18803: f64, t18807: f64, t20048: f64, t20050: f64, t20058: f64, t20417: f64, t20514: f64, t20526: f64, t2439: f64, t2829: f64, t5849: f64, t6354: f64, t64888: f64, t64923: f64, t64966: f64, t64972: f64, t64989: f64, t65013: f64, t66608: f64, t66615: f64, t66631: f64, t66641: f64) -> f64 {
    let t66897 = -3.0_f64 * t20417 * t65013 + 3.0_f64 * t2439 * t5849 * t20058 + t66615 + t1692 * t18803 * t1497 / 2.0_f64 - 3.0_f64 * t18728 * t64989 - t1692 * t20514 * t18268 + t1692 * t6354 * t2829 / 2.0_f64 + t66631 + 2.0_f64 * t66608 * t20048 + t20526 * t64888 - t66641 + 3.0_f64 * t18728 * t64966 - t1692 * t18807 * t20050 + 6.0_f64 * t18728 * t64923 - 3.0_f64 * t18728 * t64972;
    t66897
}
