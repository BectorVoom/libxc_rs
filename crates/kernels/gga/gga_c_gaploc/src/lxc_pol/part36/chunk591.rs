//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 591/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk591<F: Float>(t10847: F, t7573: F, t7572: F, t10820: F, t326: F, t825: F, t7585: F, t2684: F, t1: F, t2084: F, t106: F, t787: F) -> (F, F, F, F, F, F) {
    let t10903 = t7573 * t10847;
    let t10905 = 0.69017266717057349418e1 * t7572 * t10903;
    let t10906 = t326 * t10820;
    let t10908 = 0.92023022289409799224e1 * t825 * t10906;
    let t10909 = t7585 * t10820;
    let t10911 = 0.43710935587469654631e2 * t2684 * t10909;
    let t10912 = t2084 * t1;
    let t10913 = t10912 * t106;
    let t10914 = t787 * t10913;
    (t10905, t10908, t10911, t10912, t10913, t10914)
}
