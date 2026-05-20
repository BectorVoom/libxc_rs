//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2683/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2683<F: Float>(t6244: F, t905: F, t11774: F, t4782: F, t53391: F, t1011: F, t15993: F, t18909: F, t11933: F, t19976: F, t3115: F, t42793: F, t6272: F) -> (F, F, F, F, F) {
    let t66966 = t6244 * t905;
    let t66972 = t11774 * t53391 * t4782;
    let t66981 = t1011 * t15993 * t18909;
    let t67006 = t11933 * t19976;
    let t67015 = t3115 * t42793 * t6272;
    (t66966, t66972, t66981, t67006, t67015)
}
