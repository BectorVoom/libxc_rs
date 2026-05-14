//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 958/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk958<F: Float>(t11476: F, t3913: F, t2381: F, t10242: F, t10272: F, t10276: F, t10278: F, t11447: F, t11452: F, t11458: F, t11462: F, t11469: F, t1238: F, t1242: F, t2380: F, t3185: F, t3206: F, t3235: F, t385: F, t3860: F, t3866: F, t404: F, t8398: F) -> (F, F) {
    let t11477 = t3913 * t11476;
    let t11478 = t2381 * t11477;
    let t11481 = -t385 * t11447 / 16.0 + 0.85748036236139473944e-3 * t10242 + 0.38586616306262763276e-2 * t3235 * t11452 + 0.28582678745379824648e-3 * t8398 - 0.64311027177104605458e-3 * t3206 * t11458 - 0.12862205435420921092e-2 * t2380 * t11462 + 0.25724410870841842184e-2 * t10272 - 0.85748036236139473944e-3 * t10276 + 0.91464571985215438873e-2 * t10278 - 0.51448821741683684368e-2 * t404 * t11469 - 0.43445671692977333464e-1 * t3860 * t1242 + 0.68598428988911579154e-2 * t1238 * t3866 - 0.25724410870841842183e-2 * t3185 * t11478;
    (t11477, t11481)
}
