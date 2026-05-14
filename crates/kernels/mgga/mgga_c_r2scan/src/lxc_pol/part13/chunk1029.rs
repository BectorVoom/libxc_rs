//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1029/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1029<F: Float>(t37982: F, t7606: F, t26186: F, t3308: F, t6528: F, t10810: F, t2583: F, t574: F, t10788: F, t3613: F, t5095: F, t20825: F, t3610: F, t10776: F, t8002: F, t10772: F, t7945: F) -> (F, F, F, F, F, F, F) {
    let t39785 = t37982 * t7606;
    let t39786 = 0.19514881078765566037e-1 * t39785;
    let t39789 = t6528 * t3308 * t26186;
    let t39792 = t574 * t10810 * t2583;
    let t39793 = 0.23115257973478049502e0 * t39792;
    let t39795 = t5095 * t3613 * t10788;
    let t39801 = t20825 * t3610;
    let t39804 = t10776 * t3308 * t8002;
    let t39807 = t10772 * t3308 * t7945;
    (t39786, t39789, t39793, t39795, t39801, t39804, t39807)
}
