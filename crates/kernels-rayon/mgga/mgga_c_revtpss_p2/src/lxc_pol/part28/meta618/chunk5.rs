//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2170/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2170(t99073: f64, t14738: f64, t7038: f64, t14732: f64, t25245: f64, t93004: f64, t93008: f64, t93010: f64, t93013: f64, t93021: f64, t99063: f64, t99065: f64, t99066: f64, t99070: f64, t99071: f64) -> f64 {
    let t99074 = 0.10164000561857065645e-2_f64 * t99073;
    let t99075 = t7038 * t14738;
    let t99077 = t25245 * t14732;
    let t99078 = 0.50820002809285328226e-4_f64 * t99077;
    let t99079 = 0.57165357490759649296e-4_f64 * t93004 + t93008 - 0.28582678745379824648e-3_f64 * t93010 - t99063 / 4.0_f64 - t93013 - t99065 - t93021 - 0.80031500487063509016e-1_f64 * t99066 - t99070 + 0.17149607247227894789e-1_f64 * t99071 + t99074 - 0.42874018118069736972e-3_f64 * t99075 - t99078;
    t99079
}
