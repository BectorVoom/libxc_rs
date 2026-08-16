//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1556/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1556<F: Float>(t5326: F, t6594: F, t20973: F, t5391: F, t5381: F, t12916: F, t24735: F, t5331: F, t12855: F, t24835: F, t1038: F, t1241: F, t1244: F, t24679: F) -> (F, F, F, F, F, F) {
    let t83114 = t5326 * t6594;
    let t83130 = t5391 * t20973;
    let t83136 = t5381 * t20973;
    let t83143 = t5331 * t12916 * t24735;
    let t83158 = t12855 * t12916 * t24835;
    let t83296 = t1241 * t1244 * t24679 * t1038;
    (t83114, t83130, t83136, t83143, t83158, t83296)
}
