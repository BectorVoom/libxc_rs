//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1298/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1298<F: Float>(t24452: F, t481: F, t7469: F, t6243: F, t1604: F, t2201: F, t6263: F, t785: F, t938: F, t1554: F, t19837: F, t19841: F, t19843: F, t19990: F, t2139: F, t24433: F, t24439: F, t24447: F, t24450: F, t2579: F, t360: F, t6149: F, t7430: F, t7977: F) -> (F, F) {
    let t24453 = 0.19043987679069580388e-1 * t24452;
    let t24454 = t7469 * t481;
    let t24455 = t6243 * t24454;
    let t24456 = t1604 * t24455;
    let t24463 = t2201 * t785 * t6263 * t938;
    let t24469 = -0.69345773920434148506e0 * t24433 + 0.13002332610081402845e0 * t6149 * t7430 + 0.39006997830244208535e0 * t19990 * t2579 - 0.76830240467580968651e0 * t24439 + 0.86743646395112941037e-3 * t24447 + 0.20803732176130244552e1 * t24450 + t24453 - 0.9878173774403267398e-1 * t24456 - 0.526901789126670283e0 * t19837 - 0.11708928647259339622e0 * t19841 - 0.49390868872016336991e-1 * t19843 - 0.73613752582167450608e0 * t24463 + 0.39006997830244208535e0 * t2139 * t360 * t7977 * t1554;
    (t24455, t24469)
}
