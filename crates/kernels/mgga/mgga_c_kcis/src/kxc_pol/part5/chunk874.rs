//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 874/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk874<F: Float>(t1889: F, t1961: F, t3766: F, t1471: F, t3771: F, t6281: F, t1472: F, t6284: F, t3780: F, t6957: F, t542: F, t5463: F) -> (F, F, F, F, F, F) {
    let t7221 = t1889 * t1961;
    let t7222 = t3766 * t7221;
    let t7226 = t1471 * t3771 * t6281;
    let t7230 = t1471 * t1472 * t6284;
    let t7233 = t3780 * t6957;
    let t7234 = t542 * t7233;
    let t7237 = t5463 * t1961;
    (t7222, t7226, t7230, t7233, t7234, t7237)
}
