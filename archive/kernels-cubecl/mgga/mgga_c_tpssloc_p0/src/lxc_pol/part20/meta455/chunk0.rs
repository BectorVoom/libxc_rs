//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1911/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1911<F: Float>(t15035: F, t15238: F, t491: F, t1246: F, t15026: F, t3623: F, t11889: F, t3507: F, t1755: F, t15018: F, t3612: F, t5075: F, t5079: F) -> (F, F, F, F, F, F, F, F) {
    let t15239 = t15035 + t15238;
    let t15240 = t491 * t15239;
    let t15241 = t15240 * t1246;
    let t15245 = t15026 * t3623;
    let t15247 = t11889 * t3507;
    let t15248 = t1755 * t15247;
    let t15253 = t15018 * t3612;
    let t15257 = t5075 * t5079;
    (t15239, t15240, t15241, t15245, t15247, t15248, t15253, t15257)
}
