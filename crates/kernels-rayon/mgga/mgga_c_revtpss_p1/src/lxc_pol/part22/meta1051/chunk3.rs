//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3708/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3708(t12916: f64, t17709: f64, t20958: f64, t1012: f64, t1122: f64, t1222: f64, t1238: f64, t17280: f64, t17290: f64, t17711: f64, t1791: f64, t20747: f64, t20956: f64, t3601: f64, t3626: f64, t3699: f64, t3720: f64, t44535: f64, t44586: f64, t5320: f64, t5327: f64, t57045: f64, t57049: f64, t57265: f64, t58920: f64, t59001: f64, t59033: f64, t60717: f64, t70221: f64, t70225: f64, t70235: f64) -> f64 {
    let t70250 = t17709 * t12916 * t20958;
    let t70254 = -0.42874018118069736972e-3_f64 * t59033 * t1791 - 0.85748036236139473944e-3_f64 * t17290 * t5320 - 0.42874018118069736972e-3_f64 * t5327 * t17280 + 0.45732285992607719436e-2_f64 * t70221 * t1238 - t70225 / 972.0_f64 + t1222 * t1012 * t3699 * t60717 / 108.0_f64 + 0.12862205435420921092e-2_f64 * t17709 * t3720 * t20956 * t44586 + 0.51448821741683684368e-2_f64 * t58920 * t3720 * t70235 * t44535 * t3601 - 0.77173232612525526552e-2_f64 * t59001 * t3720 * t70235 * t17711 + 0.17149607247227894789e-2_f64 * t57265 * t3626 * t20747 * t1122 + 0.17149607247227894789e-2_f64 * t70250 - 0.57165357490759649296e-3_f64 * t57045 + 0.30488190661738479624e-2_f64 * t57049;
    t70254
}
