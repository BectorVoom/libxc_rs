//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2763/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2763<F: Float>(t22020: F, t2661: F, t5675: F, t9934: F, t22267: F, t9976: F, t13847: F, t1399: F, t73731: F, t9816: F, t22294: F, t48862: F, t48999: F) -> (F, F, F, F) {
    let t73951 = t2661 * t9934 * t22020 * t5675;
    let t73953 = t9976 * t22267;
    let t73963 = t9816 * t13847 * t73731 * t1399;
    let t73975 = t48862 * t48999 * t22294;
    (t73951, t73953, t73963, t73975)
}
