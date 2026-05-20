//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3538/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3538<F: Float>(t1025: F, t371: F, t6276: F, t676: F, t15749: F, t4858: F, t11789: F, t20016: F, t3205: F, t6337: F, t15666: F, t1053: F, t19463: F) -> (F, F, F, F, F, F) {
    let t67186 = t1025 * t371 * t676 * t6276;
    let t67195 = t4858 * t15749;
    let t67199 = t11789 * t20016;
    let t67206 = t3205 * t371 * t676 * t6337;
    let t67213 = t4858 * t15666;
    let t67215 = t19463 * t1053;
    (t67186, t67195, t67199, t67206, t67213, t67215)
}
