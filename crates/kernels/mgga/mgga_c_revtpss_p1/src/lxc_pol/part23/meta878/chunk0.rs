//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2784/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2784<F: Float>(t1444: F, t2782: F, t4075: F, t556: F, t6918: F, t22453: F, t47530: F, t5599: F, t5775: F, t689: F, t1426: F, t6889: F, t786: F) -> (F, F, F, F) {
    let t74824 = t2782 * t556 * t4075 * t6918 * t1444;
    let t74826 = t47530 * t22453;
    let t74829 = t689 * t5599 * t5775;
    let t74835 = t786 * t6889 * t1426;
    (t74824, t74826, t74829, t74835)
}
