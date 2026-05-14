//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 939/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk939<F: Float>(t378: F, t8040: F, t7930: F, t6090: F, t6093: F, t6180: F, t6183: F, t6249: F, t7947: F, t7950: F, t7955: F, t7959: F, t7961: F, t7967: F, t7979: F, t7982: F) -> (F, F, F, F) {
    let t8041 = t8040 * t378;
    let t8045 = 0.103295e1 * t7930;
    let t8054 = -t6249 + 0.13772666666666666667e1 * t6090 - 0.516475e0 * t6093 - t8045 + 0.1549425e1 * t7947 + 0.34731666666666666667e0 * t7950 + 0.3529725e1 * t7959 + 0.6311625e0 * t7961 - 0.20839e0 * t6180 - 0.20839e0 * t6183 + 0.68863333333333333333e0 * t7955 - 0.3529725e1 * t7967;
    let t8059 = 0.41678e0 * t7979;
    let t8060 = 0.41678e0 * t7982;
    (t8041, t8054, t8059, t8060)
}
