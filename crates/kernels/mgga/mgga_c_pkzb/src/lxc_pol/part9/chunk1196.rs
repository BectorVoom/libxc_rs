//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1196/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1196<F: Float>(t2368: F, t824: F, t300: F, t3175: F, t3185: F, t8381: F, t926: F, t8423: F, t8428: F, t8431: F, t54: F, t8253: F, t8260: F, t10044: F, t10213: F, t18957: F, t19033: F, t19036: F, t2185: F, t2226: F, t22952: F, t23022: F, t2371: F, t2380: F, t2381: F, t2383: F, t3187: F, t3206: F, t3235: F, t406: F, t6185: F, t6368: F, t6413: F, t6418: F, t6462: F, t6471: F, t6485: F, t6518: F, t6526: F, t758: F, t8254: F, t8319: F, t8409: F, t8418: F, t8435: F, t8450: F, t919: F) -> (F, F) {
    let t23167 = t2368 * t824;
    let t23176 = t300 * t3175;
    let t23201 = t3185 * t926 * t8381;
    let t23204 = t3185 * t926 * t8423;
    let t23207 = t8428 * t926 * t8431;
    let t23213 = t54 * t8253;
    let t23215 = t3185 * t23213 * t8260;
    let t23236 = 0.77173232612525526551e-2 * t8435 * t8254 * t6526 * t23167 + 0.77173232612525526551e-2 * t2380 * t300 * t8409 * t2383 - 0.25724410870841842183e-2 * t3185 * t23176 * t6418 + 0.12862205435420921092e-2 * t3206 * t23176 * t6471 - 0.77173232612525526551e-2 * t8428 * t8254 * t6518 * t23167 - 0.15434646522505105311e-1 * t2380 * t300 * t8418 * t6368 + 0.77173232612525526551e-2 * t3185 * t10213 * t2371 * t919 * t2226 - 0.15434646522505105311e-1 * t3235 * t758 * t8418 * t6185 + 0.17149607247227894789e-2 * t23201 + 0.85748036236139473944e-3 * t23204 + 0.25724410870841842184e-2 * t23207 + 0.12862205435420921092e-2 * t3185 * t406 * t22952 * t3187 - 0.34299214494455789577e-2 * t23215 - 0.25724410870841842183e-2 * t3185 * t8254 * t2371 * t18957 - 0.25724410870841842183e-2 * t3185 * t8254 * t2371 * t2185 * t919 + 0.68598428988911579154e-2 * t8319 * t6485 + 0.13719685797782315831e-1 * t10044 * t6413 - 0.85748036236139473944e-3 * t19033 - 0.17149607247227894789e-2 * t19036 - 0.42874018118069736972e-3 * t8450 * t2381 * t23022 * t6462;
    (t23213, t23236)
}
