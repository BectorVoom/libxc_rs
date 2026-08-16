//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 982/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk982<F: Float>(t14889: F, t319: F, t840: F, t1091: F, t2867: F, t10703: F, t2770: F, t14690: F, t4311: F, t684: F, t835: F, t4246: F) -> (F, F, F, F, F) {
    let t15222 = t840 * t319 * t14889;
    let t15225 = t1091 * t2867;
    let t15226 = t10703 * t15225;
    let t15229 = t2770 * t319;
    let t15230 = t15229 * t14690;
    let t15234 = t835 * t4311 * t684;
    let t15238 = t840 * t4246 * t2867;
    (t15222, t15226, t15230, t15234, t15238)
}
