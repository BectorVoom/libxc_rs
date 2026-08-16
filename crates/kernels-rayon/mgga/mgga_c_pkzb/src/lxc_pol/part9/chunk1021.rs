//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1021/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1021(t2380: f64, t2384: f64, t3174: f64, t3185: f64, t3206: f64, t404: f64, t6379: f64, t6383: f64, t6392: f64, t8247: f64, t8249: f64, t8256: f64, t8261: f64, t8265: f64, t8270: f64, t8275: f64, t8278: f64, t8282: f64, t8285: f64, t8312: f64, t8317: f64, t8319: f64, t918: f64) -> f64 {
    let t8322 = -t8247 - 0.42874018118069736972e-3_f64 * t404 * t8249 + t6379 + 0.19055119163586549765e-3_f64 * t6383 + 0.85748036236139473944e-3_f64 * t3206 * t8256 - 0.17149607247227894789e-2_f64 * t3185 * t8261 + 0.25724410870841842184e-2_f64 * t2380 * t8265 - 0.28582678745379824648e-3_f64 * t6392 + t3174 * t8270 / 48.0_f64 + t8275 + t3174 * t8278 / 24.0_f64 - t3174 * t8282 / 16.0_f64 + 0.2540682555144873302e-3_f64 * t8285 + 0.21437009059034868486e-3_f64 * t918 * t8312 + t8317 + 0.45732285992607719436e-2_f64 * t8319 * t2384;
    t8322
}
