//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1312/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1312(t10044: f64, t10098: f64, t10245: f64, t2364: f64, t2371: f64, t2380: f64, t2394: f64, t26975: f64, t27073: f64, t27076: f64, t28033: f64, t31790: f64, t31805: f64, t31807: f64, t31817: f64, t31820: f64, t3185: f64, t3189: f64, t3196: f64, t3209: f64, t3224: f64, t8254: f64, t8319: f64) -> f64 {
    let t31822 = -0.51448821741683684366e-2_f64 * t3185 * t8254 * t2371 * t31790 + 0.38586616306262763276e-2_f64 * t2380 * t28033 * t3224 + 0.68598428988911579154e-2_f64 * t8319 * t10245 - 0.68598428988911579154e-2_f64 * t10044 * t10098 + 0.57165357490759649297e-3_f64 * t27073 - 0.42874018118069736972e-3_f64 * t27076 + 0.85748036236139473947e-3_f64 * t31805 + 0.43445671692977333464e-1_f64 * t2364 * t31807 * t3189 - 0.21722835846488666732e-1_f64 * t2394 * t31807 * t3209 - 0.43445671692977333464e-1_f64 * t26975 * t3196 + 0.25724410870841842183e-2_f64 * t31817 - 0.17149607247227894789e-2_f64 * t31820;
    t31822
}
