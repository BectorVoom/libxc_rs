//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1309/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1309(t10275: f64, t1238: f64, t10197: f64, t10204: f64, t10208: f64, t10214: f64, t11456: f64, t22469: f64, t22475: f64, t22945: f64, t22951: f64, t23054: f64, t2380: f64, t2396: f64, t2411: f64, t26970: f64, t26981: f64, t27020: f64, t28147: f64, t300: f64, t3185: f64, t3202: f64, t3206: f64, t3880: f64, t824: f64, t8254: f64, t8450: f64) -> f64 {
    let t31755 = t1238 * t10275;
    let t31765 = -0.85748036236139473944e-3_f64 * t22469 + 0.38586616306262763276e-2_f64 * t2380 * t300 * t2411 * t3880 * t10214 - 0.25724410870841842184e-2_f64 * t3185 * t27020 * t10208 + 0.12862205435420921092e-2_f64 * t3206 * t27020 * t10204 + 0.12862205435420921092e-2_f64 * t3206 * t8254 * t2396 * t3880 * t824 + t22475 + 0.45732285992607719436e-2_f64 * t31755 + 0.64311027177104605458e-3_f64 * t8450 * t23054 * t28147 * t11456 + 0.21722835846488666732e-1_f64 * t10197 * t3202 + 0.85748036236139473944e-3_f64 * t26970 + t22945 + t22951 - 0.17149607247227894789e-2_f64 * t26981;
    t31765
}
