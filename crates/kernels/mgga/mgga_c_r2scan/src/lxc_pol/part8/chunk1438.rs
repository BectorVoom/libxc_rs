//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1438/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1438<F: Float>(t2858: F, t2892: F, t6955: F, t2266: F, t2850: F, t2867: F, t910: F, t3016: F, t23829: F, t23835: F, t32968: F, t32969: F, t32970: F, t32971: F, t32972: F, t34837: F, t34840: F) -> (F, F, F, F) {
    let t34843 = 18.0 * t2858 * t6955 * t2892;
    let t34847 = 18.0 * t2266 * t2867 * t910 * t2850;
    let t34850 = 9.0 * t2266 * t6955 * t3016;
    let t34851 = -t23829 + t32968 - t23835 - t32969 - t34837 - t34840 - t32970 - t34843 + t34847 - t32971 - t32972 - t34850;
    (t34843, t34847, t34850, t34851)
}
