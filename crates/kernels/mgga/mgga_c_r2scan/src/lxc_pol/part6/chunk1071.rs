//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1071/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1071<F: Float>(t1751: F, t4970: F, t41: F, t424: F, t4878: F, t458: F, t4885: F, t4911: F, t1379: F, t1496: F, t1381: F) -> (F, F, F, F, F, F) {
    let t18891 = t1751 * t4970;
    let t18894 = t41 * t424 * t4878;
    let t18896 = t4885 * t458;
    let t18900 = t4911 * t458;
    let t18903 = 1.0 / t1379 / t1496;
    let t18904 = t1381 * t1381;
    (t18891, t18894, t18896, t18900, t18903, t18904)
}
