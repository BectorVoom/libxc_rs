//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1310/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1310(t11347: f64, t2099: f64, t6459: f64, t10241: f64, t8368: f64, t10271: f64, t1238: f64, t2411: f64, t300: f64, t3874: f64, t10047: f64, t10067: f64, t22989: f64, t23008: f64, t2371: f64, t26986: f64, t26995: f64, t27007: f64, t27014: f64, t27028: f64, t27031: f64, t3061: f64, t3185: f64) -> (f64, f64) {
    let t31771 = t6459 * t2099 * t11347;
    let t31773 = t8368 * t10241;
    let t31777 = t1238 * t10271;
    let t31782 = t300 * t2411 * t3874;
    let t31787 = -t22989 - t26986 / 9.0_f64 + t26995 / 48.0_f64 - 0.85748036236139473944e-3_f64 * t27007 - t23008 + 0.42874018118069736972e-3_f64 * t27014 + 0.14291339372689912324e-3_f64 * t31771 - 0.45732285992607719436e-2_f64 * t31773 - 0.68598428988911579154e-2_f64 * t10047 * t10067 - 0.13719685797782315831e-1_f64 * t31777 - t27028 / 16.0_f64 + t27031 / 24.0_f64 + 0.77173232612525526549e-2_f64 * t3185 * t31782 * t2371 * t3061;
    (t31782, t31787)
}
