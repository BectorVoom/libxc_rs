//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 797/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk797<F: Float>(t12001: F, t3852: F, t1168: F, t2373: F, t2574: F, t762: F, t2569: F, t10052: F, t242: F, t10085: F, t3898: F, t11593: F, t14095: F, t14100: F, t14105: F, t14110: F, t14114: F, t14118: F, t14122: F, t14126: F, t14130: F, t14135: F, t1901: F, t446: F, t9982: F) -> (F, F) {
    let t14138 = t12001 * t3852;
    let t14140 = t1168 * t2373;
    let t14142 = t2574 * t762 * t14140;
    let t14145 = t1168 * t2569;
    let t14146 = t10052 * t14145;
    let t14147 = t242 * t14146;
    let t14150 = t10085 * t3898;
    let t14153 = 2.0 / 9.0 * t1901 * t14095 + 4.0 / 9.0 * t1901 * t14100 + t1901 * t14105 / 9.0 + 4.0 / 3.0 * t446 * t14110 + 4.0 / 27.0 * t14114 - t9982 - 8.0 / 9.0 * t11593 * t14118 + 8.0 / 27.0 * t11593 * t14122 - t14126 - 4.0 / 3.0 * t1901 * t14130 - 2.0 * t446 * t14135 - 22.0 / 27.0 * t14138 - 2.0 / 3.0 * t446 * t14142 - 2.0 * t446 * t14147 + 2.0 / 9.0 * t1901 * t14150;
    (t14146, t14153)
}
