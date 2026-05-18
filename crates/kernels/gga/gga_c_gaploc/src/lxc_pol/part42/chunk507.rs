//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 507/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk507<F: Float>(t5345: F, t5348: F, t9106: F, t2519: F, t3220: F, t3225: F, t716: F, t2524: F, t871: F, t1890: F, t723: F, t2558: F, t7634: F) -> (F, F, F, F, F, F) {
    let t9672 = t5345 * t9106 * t5348;
    let t9674 = t3220 * t2519;
    let t9676 = t3225 * t716;
    let t9682 = t2524 * t871;
    let t9740 = t1890 * t723;
    let t9752 = t7634 * t2558;
    (t9672, t9674, t9676, t9682, t9740, t9752)
}
