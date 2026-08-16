//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 764/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk764(t7545: f64, t7549: f64, t7557: f64, t7601: f64, t7611: f64, t7551: f64, t7562: f64, t7567: f64, t7571: f64, t7573: f64, t7578: f64, t7581: f64, t7589: f64, t7593: f64, t7597: f64, t7603: f64, t7606: f64, t7608: f64, t7615: f64, t7617: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8192 = 0.31448092289604152069e-3_f64 * t7545;
    let t8193 = 0.41930789719472202758e-3_f64 * t7549;
    let t8195 = 0.62896184579208304138e-3_f64 * t7557;
    let t8205 = 0.13073958333333333333e0_f64 * t7601;
    let t8209 = 0.21437009059034868486e-3_f64 * t7611;
    let t8212 = t8192 + t8193 - 0.18868855373762491241e-1_f64 * t7551 - t8195 + 0.68598428988911579156e-2_f64 * t7562 + 0.37737710747524982482e-2_f64 * t7567 + 0.85748036236139473944e-3_f64 * t7571 + 0.25724410870841842184e-2_f64 * t7573 + 0.42874018118069736972e-2_f64 * t7578 - 0.42874018118069736972e-3_f64 * t7581 - 0.28582678745379824648e-3_f64 * t7589 - t7593 / 192.0_f64 - 0.7640625e-2_f64 * t7597 - t8205 + 0.17149607247227894789e-2_f64 * t7603 - 0.34299214494455789578e-2_f64 * t7606 - 0.17149607247227894789e-2_f64 * t7608 + t8209 + 0.32012600194825403606e-1_f64 * t7615 - 0.16006300097412701803e-1_f64 * t7617;
    (t8192, t8193, t8195, t8205, t8209, t8212)
}
