//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1397/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1397<F: Float>(t1416: F, t2794: F, t21298: F, t21299: F, t21301: F, t21309: F, t21313: F, t21315: F, t21320: F, t21322: F, t21324: F, t21326: F, t21330: F, t21333: F, t21335: F, t22428: F, t2743: F) -> (F, F) {
    let t26463 = t1416 * t2794;
    let t26468 = -t21298 - 0.28895839882605942647e1 * t21299 - 0.29277669510814697573e0 * t21301 + t21309 - t21313 - 0.11711067804325879029e1 * t21315 + 0.5848223622634646207e0 * t21320 + 0.17544670867903938621e1 * t21322 - 60.0 * t26463 - 0.30015321757399999999e-2 * t21324 + 0.86687519647817827941e1 * t21326 + t21330 + t21333 - 0.60030643514799999999e-2 * t21335;
    let t26475 = t2743 * t22428;
    (t26468, t26475)
}
