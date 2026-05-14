//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1219/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1219<F: Float>(t26020: F, t546: F, t9520: F, t6407: F, t8074: F, t8078: F, t20481: F, t551: F, t574: F, t921: F, t2196: F, t2625: F, t6343: F, t20090: F, t20820: F, t923: F) -> (F, F, F, F, F, F, F) {
    let t26021 = 0.17563392970889009434e0 * t26020;
    let t26029 = t546 * t9520;
    let t26036 = t6407 * t8074;
    let t26037 = 0.17563392970889009434e0 * t26036;
    let t26038 = t6407 * t8078;
    let t26039 = 0.87816964854445047168e-1 * t26038;
    let t26042 = t574 * t551 * t20481 * t921;
    let t26052 = t2196 * t551 * t6343 * t2625;
    let t26053 = 0.15256070262495512671e2 * t26052;
    let t26060 = t20090 * t923 * t20820;
    (t26021, t26029, t26037, t26039, t26042, t26053, t26060)
}
