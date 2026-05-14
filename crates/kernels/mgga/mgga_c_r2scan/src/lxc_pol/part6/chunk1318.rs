//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1318/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1318<F: Float>(t259: F, t7564: F, t546: F, t565: F, t2139: F, t2294: F, t7449: F, t20296: F, t20301: F, t20308: F, t20311: F, t20316: F, t20321: F, t20324: F, t20328: F, t20582: F, t2136: F, t2142: F, t24875: F, t24877: F, t2582: F, t360: F, t495: F, t8050: F) -> (F,) {
    let t24882 = t7564 * t259;
    let t24883 = t546 * t24882;
    let t24886 = t565 * t24882;
    let t24890 = t2139 * t2294 * t7449;
    let t24899 = -0.7801399566048841707e1 * t20582 * t8050 - 0.19043987679069580389e-1 * t20296 - 0.20803732176130244552e1 * t24875 - 0.13002332610081402845e0 * t2582 * t360 * t24877 * t495 + 0.26004665220162805689e0 * t24883 * t2136 + 0.7801399566048841707e0 * t24886 * t2142 - 0.20803732176130244552e1 * t24890 - 0.34930954652346593433e-1 * t20301 - 0.1047928639570397803e0 * t20308 + 0.34930954652346593433e-1 * t20311 + 0.17465477326173296717e-1 * t20316 + 0.52396431978519890151e-1 * t20321 + 0.29272321618148349056e-1 * t20324 + 0.12459097221822660494e0 * t20328;
    (t24899,)
}
