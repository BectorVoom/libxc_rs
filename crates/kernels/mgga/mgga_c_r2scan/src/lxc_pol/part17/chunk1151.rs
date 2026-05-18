//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1151/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1151<F: Float>(t2983: F, t352: F, t856: F, t12574: F, t481: F, t983: F, t986: F, t792: F, t39299: F, t12428: F, t35373: F, t910: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42757 = t2983 * t856 * t352;
    let t42819 = t12574 * t481;
    let t42846 = t983 * t986;
    let t42847 = t42846 * t792;
    let t42851 = t39299 * t986;
    let t42855 = t12428 * t481;
    let t42863 = t35373 * t481;
    let t42868 = t35373 * t792;
    let t42877 = t910 * t983;
    (t42757, t42819, t42846, t42847, t42851, t42855, t42863, t42868, t42877)
}
