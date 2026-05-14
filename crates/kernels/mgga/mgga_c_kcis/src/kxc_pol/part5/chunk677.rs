//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 677/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk677<F: Float>(t1856: F, t3622: F, t1267: F, t1846: F, t3500: F, t1251: F, t2888: F, t421: F, t4567: F, t1262: F, t1662: F, t3515: F, t993: F, t4581: F, t737: F, t992: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5281 = t1856 * t3622;
    let t5282 = t5281 * t1267;
    let t5299 = t3500 * t1846;
    let t5300 = t1251 * t5299;
    let t5302 = t2888 * t421;
    let t5303 = t5302 * t4567;
    let t5306 = t1662 * t1262;
    let t5307 = t3515 * t5306;
    let t5310 = t993 * t421;
    let t5311 = t5310 * t4581;
    let t5314 = t737 * t992;
    (t5281, t5282, t5299, t5300, t5302, t5303, t5306, t5307, t5310, t5311, t5314)
}
