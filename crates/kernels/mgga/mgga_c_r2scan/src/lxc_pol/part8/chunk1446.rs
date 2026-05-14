//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1446/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1446<F: Float>(t18920: F, t18922: F, t18930: F, t18934: F, t18941: F, t18973: F, t18979: F, t18984: F, t23719: F, t32125: F, t32127: F, t2368: F, t9634: F, t10439: F, t1256: F, t810: F) -> (F, F, F) {
    let t34916 = t18920 - t18922 - t18930 + t32125 - t32127 + t18934 - t18941 + t23719 + t18973 - t18979 + t18984;
    let t34919 = t2368 * t9634;
    let t34923 = t1256 * t10439 * t810;
    (t34916, t34919, t34923)
}
