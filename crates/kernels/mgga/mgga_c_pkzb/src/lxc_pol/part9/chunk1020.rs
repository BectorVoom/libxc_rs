//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1020/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1020<F: Float>(t2003: F, t465: F, t179: F, t1885: F, t299: F, t53: F, t5633: F, t5635: F, t2002: F, t220: F, t5629: F, t771: F, t5680: F, t2068: F, t5537: F, t2070: F, t2082: F) -> (F, F, F, F, F, F, F) {
    let t18199 = t465 * t2003;
    let t18202 = t299 * t179 * t18199 * t1885;
    let t18204 = t53 * t5633;
    let t18207 = t299 * t179 * t18204 * t5635;
    let t18210 = 1.0 / t2002 / t220;
    let t18216 = t771 * t5629;
    let t18218 = t771 * t5680;
    let t18232 = t299 * t179 * t2068 * t5537;
    let t18234 = t2082 * t2070;
    (t18202, t18207, t18210, t18216, t18218, t18232, t18234)
}
