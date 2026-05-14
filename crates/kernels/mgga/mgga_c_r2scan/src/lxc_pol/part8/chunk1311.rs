//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1311/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1311<F: Float>(t406: F, t9905: F, t410: F, t18916: F, t18920: F, t18922: F, t18930: F, t18934: F, t18941: F, t18973: F, t18979: F, t19445: F, t23719: F, t23732: F, t22335: F, t22344: F, t26803: F, t26804: F) -> (F, F, F, F) {
    let t32124 = t406 * t9905;
    let t32125 = 4.0 * t32124;
    let t32126 = t410 * t9905;
    let t32127 = 4.0 * t32126;
    let t32128 = -t18916 - t18920 - 0.2363e1 * t19445 + t18922 + t18930 - t32125 + t32127 - t18934 + t18941 - t23719 - t18973 + t18979 - t23732;
    let t32131 = t22335 + t26803 - t26804 - t22344;
    (t32125, t32127, t32128, t32131)
}
