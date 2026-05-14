//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1399/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1399<F: Float>(t3730: F, t6404: F, t5728: F, t919: F, t154: F, t18989: F, t3757: F, t385: F, t6446: F, t10070: F, t10204: F, t10258: F, t19055: F, t19067: F, t19140: F, t2226: F, t23054: F, t23130: F, t23254: F, t2352: F, t2367: F, t2371: F, t2382: F, t2396: F, t27044: F, t27057: F, t27237: F, t27254: F, t3206: F, t3235: F, t6526: F, t758: F, t8254: F, t8349: F, t8420: F, t8435: F, t8450: F) -> (F,) {
    let t28138 = t6404 * t3730;
    let t28147 = t5728 * t919;
    let t28166 = t385 * t154 * t18989 * t3757;
    let t28174 = t385 * t154 * t6446 * t3730;
    let t28181 = 0.25724410870841842184e-1 * t3235 * t758 * t19140 * t3757 * t2226 + 0.54878743191129263324e-1 * t10258 * t8420 - 0.51448821741683684368e-2 * t3235 * t758 * t28138 * t2226 + 0.17149607247227894789e-2 * t3206 * t8254 * t2396 * t27057 - 0.85748036236139473944e-3 * t8450 * t27044 * t28147 * t2382 + 0.85748036236139473944e-3 * t8450 * t23054 * t28147 * t10070 + 0.51448821741683684367e-2 * t8435 * t8254 * t6526 * t8349 + 0.17149607247227894789e-2 * t3206 * t23130 * t10204 + 0.34299214494455789578e-2 * t23254 - t28166 / 216.0 + t385 * t154 * t2352 * t27254 / 24.0 + t28174 / 432.0 + 0.85748036236139473944e-3 * t2367 * t758 * t27237 * t2371 - t19055 - 0.47637797908966374413e-4 * t19067;
    (t28181,)
}
