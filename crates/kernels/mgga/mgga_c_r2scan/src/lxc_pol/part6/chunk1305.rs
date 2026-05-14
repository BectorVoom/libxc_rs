//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1305/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1305<F: Float>(t6118: F, t7505: F, t1569: F, t5065: F, t7937: F, t8289: F, t2294: F, t2582: F, t7453: F, t7940: F, t20035: F, t20038: F, t20043: F, t20049: F, t2124: F, t2557: F, t2562: F, t2590: F, t360: F, t6364: F, t6428: F, t6435: F, t7439: F, t7512: F, t7517: F, t7941: F, t8022: F) -> (F, F) {
    let t24622 = t6118 * t7505;
    let t24624 = t1569 * t5065;
    let t24639 = t8289 * t7937;
    let t24642 = t2582 * t2294 * t7453;
    let t24645 = t2582 * t2294 * t7940;
    let t24657 = -0.76830240467580968651e0 * t24622 + 0.54878743191129263322e-1 * t2557 * t2124 * t2590 * t24624 - 0.7801399566048841707e0 * t7512 * t360 * t2562 * t6428 - 0.7801399566048841707e0 * t7512 * t360 * t2562 * t6435 + 0.26004665220162805689e0 * t8022 * t7439 + 0.69345773920434148506e0 * t24639 + 0.69345773920434148506e0 * t24642 + 0.34672886960217074253e0 * t24645 - 0.13002332610081402845e0 * t8289 * t7941 - 0.98781737744032673979e0 * t2557 * t2124 * t7517 * t6364 - 0.38415120233790484326e0 * t20035 - 0.34672886960217074253e0 * t20038 - 0.69345773920434148506e0 * t20043 - 0.20803732176130244552e1 * t20049;
    (t24624, t24657)
}
