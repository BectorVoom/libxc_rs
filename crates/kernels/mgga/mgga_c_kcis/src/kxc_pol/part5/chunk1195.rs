//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1195/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1195<F: Float>(t3255: F, t7230: F, t5463: F, t5526: F, t3786: F, t1471: F, t1472: F, t18431: F, t544: F, t6957: F, t1319: F, t16411: F, t518: F, t1419: F, t5457: F, t5458: F, t5481: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22149 = t3255 * t7230;
    let t22151 = t5463 * t5526;
    let t22152 = t3786 * t22151;
    let t22156 = t1471 * t1472 * t18431;
    let t22159 = t544 * t6957;
    let t22160 = t22159 * t1319;
    let t22161 = t16411 * t22160;
    let t22164 = t518 * t6957;
    let t22165 = t22164 * t1419;
    let t22166 = t5457 * t22165;
    let t22169 = t5458 * t5481;
    (t22149, t22151, t22152, t22156, t22160, t22161, t22165, t22166, t22169)
}
