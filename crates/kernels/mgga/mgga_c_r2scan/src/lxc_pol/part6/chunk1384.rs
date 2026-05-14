//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1384/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1384<F: Float>(t20137: F, t6209: F, t8128: F, t2666: F, t6448: F, t19877: F, t25562: F, t6086: F, t19890: F, t6093: F, t7619: F, t2115: F, t6359: F, t2155: F, t25697: F, t25466: F, t8088: F) -> (F, F, F, F, F, F) {
    let t26249 = t6209 * t20137 * t8128;
    let t26251 = t6448 * t2666;
    let t26255 = t19877 * t6086 * t25562;
    let t26258 = t6093 * t19890 * t7619;
    let t26259 = 0.6112917064160653851e0 * t26258;
    let t26260 = t2115 * t6359;
    let t26262 = t2155 * t26260 * t25697;
    let t26265 = t2155 * t8088 * t25466;
    (t26249, t26251, t26255, t26259, t26262, t26265)
}
