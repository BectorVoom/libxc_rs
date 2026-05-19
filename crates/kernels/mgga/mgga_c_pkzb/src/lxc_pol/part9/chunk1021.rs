//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1021/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1021<F: Float>(t2380: F, t2384: F, t3174: F, t3185: F, t3206: F, t404: F, t6379: F, t6383: F, t6392: F, t8247: F, t8249: F, t8256: F, t8261: F, t8265: F, t8270: F, t8275: F, t8278: F, t8282: F, t8285: F, t8312: F, t8317: F, t8319: F, t918: F) -> F {
    let t8322 = -t8247 - F::cast_from(0.42874018118069736972e-3_f64) * t404 * t8249 + t6379 + F::cast_from(0.19055119163586549765e-3_f64) * t6383 + F::cast_from(0.85748036236139473944e-3_f64) * t3206 * t8256 - F::cast_from(0.17149607247227894789e-2_f64) * t3185 * t8261 + F::cast_from(0.25724410870841842184e-2_f64) * t2380 * t8265 - F::cast_from(0.28582678745379824648e-3_f64) * t6392 + t3174 * t8270 / F::new(48.0) + t8275 + t3174 * t8278 / F::new(24.0) - t3174 * t8282 / F::new(16.0) + F::cast_from(0.2540682555144873302e-3_f64) * t8285 + F::cast_from(0.21437009059034868486e-3_f64) * t918 * t8312 + t8317 + F::cast_from(0.45732285992607719436e-2_f64) * t8319 * t2384;
    t8322
}
