//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 750/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk750(t7864: f64, t1090: f64, t1181: f64, t604: f64, t7575: f64, t1096: f64, t1165: f64, t7351: f64, t7806: f64, t7809: f64, t7813: f64, t7817: f64, t7820: f64, t7823: f64, t7825: f64, t7829: f64, t7833: f64, t7837: f64, t7840: f64, t7845: f64, t7848: f64, t7850: f64, t7854: f64, t7856: f64, t7863: f64) -> (f64, f64, f64) {
    let t7865 = 7.0_f64 / 144.0_f64 * t7864;
    let t7867 = t1181 * t604 * t1090;
    let t7868 = t7575 * t7867;
    let t7871 = t1165 * t7351 * t1096;
    let t7872 = t7575 * t7871;
    let t7874 = -t7806 + 0.114609375e-1_f64 * t7809 + 0.7640625e-2_f64 * t7813 + t7817 / 64.0_f64 + 0.22921875e-1_f64 * t7820 - 0.17149607247227894789e-2_f64 * t7823 + 0.17149607247227894789e-2_f64 * t7825 - t7829 / 128.0_f64 + 0.15724046144802076034e-3_f64 * t7833 + 0.21437009059034868486e-3_f64 * t7837 + 0.31448092289604152068e-3_f64 * t7840 + 0.20965394859736101378e-3_f64 * t7845 - t7848 + t7850 + t7854 + t7856 / 96.0_f64 + t7863 - t7865 + 0.31448092289604152068e-2_f64 * t7868 - 0.47172138434406228102e-2_f64 * t7872;
    (t7867, t7871, t7874)
}
