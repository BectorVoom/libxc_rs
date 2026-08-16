//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1315/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1315<F: Float>(t23213: F, t3185: F, t8260: F, t10044: F, t10213: F, t18957: F, t19033: F, t19036: F, t2185: F, t2226: F, t22952: F, t23022: F, t23167: F, t23176: F, t23201: F, t23204: F, t23207: F, t2371: F, t2380: F, t2381: F, t2383: F, t300: F, t3187: F, t3206: F, t3235: F, t406: F, t6185: F, t6368: F, t6413: F, t6418: F, t6462: F, t6471: F, t6485: F, t6518: F, t6526: F, t758: F, t8254: F, t8319: F, t8409: F, t8418: F, t8428: F, t8435: F, t8450: F, t919: F) -> F {
    let t23215 = t3185 * t23213 * t8260;
    let t23236 = F::cast_from(0.77173232612525526551e-2_f64) * t8435 * t8254 * t6526 * t23167 + F::cast_from(0.77173232612525526551e-2_f64) * t2380 * t300 * t8409 * t2383 - F::cast_from(0.25724410870841842183e-2_f64) * t3185 * t23176 * t6418 + F::cast_from(0.12862205435420921092e-2_f64) * t3206 * t23176 * t6471 - F::cast_from(0.77173232612525526551e-2_f64) * t8428 * t8254 * t6518 * t23167 - F::cast_from(0.15434646522505105311e-1_f64) * t2380 * t300 * t8418 * t6368 + F::cast_from(0.77173232612525526551e-2_f64) * t3185 * t10213 * t2371 * t919 * t2226 - F::cast_from(0.15434646522505105311e-1_f64) * t3235 * t758 * t8418 * t6185 + F::cast_from(0.17149607247227894789e-2_f64) * t23201 + F::cast_from(0.85748036236139473944e-3_f64) * t23204 + F::cast_from(0.25724410870841842184e-2_f64) * t23207 + F::cast_from(0.12862205435420921092e-2_f64) * t3185 * t406 * t22952 * t3187 - F::cast_from(0.34299214494455789577e-2_f64) * t23215 - F::cast_from(0.25724410870841842183e-2_f64) * t3185 * t8254 * t2371 * t18957 - F::cast_from(0.25724410870841842183e-2_f64) * t3185 * t8254 * t2371 * t2185 * t919 + F::cast_from(0.68598428988911579154e-2_f64) * t8319 * t6485 + F::cast_from(0.13719685797782315831e-1_f64) * t10044 * t6413 - F::cast_from(0.85748036236139473944e-3_f64) * t19033 - F::cast_from(0.17149607247227894789e-2_f64) * t19036 - F::cast_from(0.42874018118069736972e-3_f64) * t8450 * t2381 * t23022 * t6462;
    t23236
}
