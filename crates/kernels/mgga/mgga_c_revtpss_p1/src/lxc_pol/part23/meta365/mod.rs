//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1682;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1683;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta365<F: Float>(t15191: F, t15197: F, t4682: F, t964: F, t1626: F, t3011: F, t15125: F, t11387: F, t1609: F, t4644: F, t945: F, t1614: F, t2967: F, t2986: F, t4587: F, t914: F, t1596: F, t2923: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15322, t15324, t15343, t15350, t15363, t15364, t15396, t15400, t15406) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1682::<F>(t15191, t15197, t4682, t964, t1626, t3011, t15125, t11387, t1609, t4644, t945, t1614, t2967);
        let (t15413, t15416, t15421) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1683::<F>(t1626, t2986, t4587, t914, t1596, t2923);
    (t15322, t15324, t15343, t15350, t15363, t15364, t15396, t15400, t15406, t15413, t15416, t15421)
}
