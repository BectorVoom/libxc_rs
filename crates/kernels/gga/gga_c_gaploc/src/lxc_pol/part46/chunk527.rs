//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 527/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk527<F: Float>(t3295: F, t7354: F, t2684: F, t1: F, t9636: F, t787: F, t9755: F, t2365: F, t7069: F, t7390: F, t531: F, t9689: F, t3270: F, t769: F, t314: F, t9688: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10029 = t7354 * t3295;
    let t10030 = t2684 * t10029;
    let t10031 = 0.51123901271894332901e0 * t10030;
    let t10032 = t9636 * t1;
    let t10033 = t787 * t10032;
    let t10036 = t9755 * t1;
    let t10037 = t787 * t10036;
    let t10040 = t2365 * t7069;
    let t10042 = 0.29792074959875355558e-1 * t7390 * t10040;
    let t10043 = t531 * t9689;
    let t10050 = t769 * t3270;
    let t10053 = t314 * t9688;
    (t10031, t10032, t10033, t10036, t10037, t10040, t10042, t10043, t10050, t10053)
}
