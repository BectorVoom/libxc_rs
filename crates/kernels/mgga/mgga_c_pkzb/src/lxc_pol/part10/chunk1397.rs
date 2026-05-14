//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1397/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1397<F: Float>(t10261: F, t300: F, t10220: F, t2380: F, t6475: F, t8319: F, t8470: F, t178: F, t22919: F, t6515: F, t10044: F, t10054: F, t10208: F, t1249: F, t19182: F, t23130: F, t23149: F, t23201: F, t23204: F, t2371: F, t2381: F, t2383: F, t3185: F, t3265: F, t3898: F, t3913: F, t394: F, t6366: F, t7945: F, t8254: F, t8265: F, t8424: F, t8432: F, t8473: F, t8532: F) -> (F, F) {
    let t28040 = t300 * t10261;
    let t28059 = t2380 * t6475 * t10220;
    let t28061 = t8319 * t8470;
    let t28063 = t22919 * t178;
    let t28064 = t6515 * t28063;
    let t28082 = -0.27439371595564631662e-1 * t8319 * t8265 - 0.10289764348336736874e-1 * t2380 * t28040 * t2383 + 0.28582678745379824648e-3 * t23149 - 0.34299214494455789578e-2 * t3185 * t8254 * t2371 * t8473 - 0.34299214494455789578e-2 * t3185 * t23130 * t10208 + 0.25724410870841842183e-2 * t3185 * t6366 * t3913 * t19182 - 0.45732285992607719436e-2 * t10044 * t8424 - 0.57165357490759649296e-3 * t28059 + 0.60976381323476959248e-2 * t28061 - 0.13719685797782315831e-1 * t28064 * t8432 - 0.85748036236139473944e-3 * t2380 * t2381 * t1249 * t394 * t7945 - 0.85748036236139473944e-3 * t2380 * t2381 * t8532 * t3898 - 0.17149607247227894789e-2 * t2380 * t2381 * t3265 * t10054 + 0.11433071498151929859e-2 * t23201 + 0.57165357490759649296e-3 * t23204;
    (t28063, t28082)
}
