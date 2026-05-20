//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2711/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2711<F: Float>(t1247: F, t20902: F, t3172: F, t1234: F, t21271: F, t17209: F, t17505: F, t12855: F, t12916: F, t21120: F, t21093: F, t372: F) -> (F, F, F, F, F) {
    let t69793 = t1247 * t3172 * t20902;
    let t69795 = t1234 * t21271;
    let t69812 = t17505 * t17209;
    let t69820 = t12855 * t12916 * t21120;
    let t69832 = t372 * t21093;
    (t69793, t69795, t69812, t69820, t69832)
}
