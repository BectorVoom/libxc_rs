//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1401/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1401<F: Float>(t3199: F, t919: F, t10212: F, t10214: F, t2380: F, t54: F, t10208: F, t23213: F, t3185: F, t10204: F, t3206: F, t10075: F, t18980: F, t19158: F, t19163: F, t23054: F, t23286: F, t23296: F, t23299: F, t23311: F, t23313: F, t23317: F, t3214: F, t406: F, t6518: F, t6526: F, t8312: F, t8428: F, t8435: F) -> (F,) {
    let t28213 = t3199 * t919;
    let t28227 = t2380 * t54 * t10212 * t10214;
    let t28231 = t3185 * t23213 * t10208;
    let t28234 = t3206 * t23213 * t10204;
    let t28238 = -0.28582678745379824648e-3 * t19158 + 0.12862205435420921092e-2 * t8428 * t406 * t10075 * t18980 - 0.22866142996303859718e-2 * t3214 * t8312 - t19163 + 0.51448821741683684367e-2 * t8428 * t23054 * t6518 * t28213 - 0.51448821741683684367e-2 * t8435 * t23054 * t6526 * t28213 - 0.68598428988911579156e-2 * t23286 + 0.34299214494455789578e-2 * t23296 + 0.17149607247227894789e-2 * t23299 + 0.34299214494455789578e-2 * t28227 + 0.11433071498151929859e-2 * t23311 - 0.22866142996303859718e-2 * t28231 + 0.11433071498151929859e-2 * t28234 - t23313 / 27.0 + t23317 / 108.0;
    (t28238,)
}
