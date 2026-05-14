//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 848/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk848<F: Float>(t10703: F, t15225: F, t2770: F, t319: F, t14690: F, t4311: F, t684: F, t835: F, t2867: F, t4246: F, t840: F, t3746: F, t882: F, t15138: F, t296: F, t1212: F, t2894: F) -> (F, F, F, F, F, F, F) {
    let t15226 = t10703 * t15225;
    let t15229 = t2770 * t319;
    let t15230 = t15229 * t14690;
    let t15234 = t835 * t4311 * t684;
    let t15238 = t840 * t4246 * t2867;
    let t15242 = t835 * t882 * t3746;
    let t15245 = t296 * t15138;
    let t15249 = t840 * t2894 * t1212;
    (t15226, t15230, t15234, t15238, t15242, t15245, t15249)
}
