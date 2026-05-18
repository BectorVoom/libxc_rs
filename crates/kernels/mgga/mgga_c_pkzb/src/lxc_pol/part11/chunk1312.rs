//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1312/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1312<F: Float>(t10044: F, t10098: F, t10245: F, t2364: F, t2371: F, t2380: F, t2394: F, t26975: F, t27073: F, t27076: F, t28033: F, t31790: F, t31805: F, t31807: F, t31817: F, t31820: F, t3185: F, t3189: F, t3196: F, t3209: F, t3224: F, t8254: F, t8319: F) -> F {
    let t31822 = -F::new(0.51448821741683684366e-2) * t3185 * t8254 * t2371 * t31790 + F::new(0.38586616306262763276e-2) * t2380 * t28033 * t3224 + F::new(0.68598428988911579154e-2) * t8319 * t10245 - F::new(0.68598428988911579154e-2) * t10044 * t10098 + F::new(0.57165357490759649297e-3) * t27073 - F::new(0.42874018118069736972e-3) * t27076 + F::new(0.85748036236139473947e-3) * t31805 + F::new(0.43445671692977333464e-1) * t2364 * t31807 * t3189 - F::new(0.21722835846488666732e-1) * t2394 * t31807 * t3209 - F::new(0.43445671692977333464e-1) * t26975 * t3196 + F::new(0.25724410870841842183e-2) * t31817 - F::new(0.17149607247227894789e-2) * t31820;
    t31822
}
