//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2688/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2688<F: Float>(t16095: F, t20100: F, t43131: F, t11922: F, t20069: F, t4899: F, t20065: F, t4892: F, t15688: F, t16584: F, t15731: F, t4879: F) -> (F, F, F, F, F) {
    let t67358 = t16095 * t43131 * t20100;
    let t67426 = t4899 * t11922 * t20069;
    let t67435 = t4892 * t11922 * t20065;
    let t67458 = t16584 * t15688;
    let t67473 = t4879 * t15731;
    (t67358, t67426, t67435, t67458, t67473)
}
