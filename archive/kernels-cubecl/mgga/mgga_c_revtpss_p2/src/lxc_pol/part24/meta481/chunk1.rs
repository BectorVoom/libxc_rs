//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1471/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1471<F: Float>(t1263: F, t372: F, t6628: F, t1260: F, t20850: F, t11262: F, t3600: F, t6630: F, t3610: F, t6634: F, t5326: F, t5390: F) -> (F, F, F, F, F) {
    let t69839 = t372 * t1263 * t6628;
    let t69906 = t20850 * t1260;
    let t69910 = t3600 * t11262 * t6630;
    let t69964 = t3610 * t11262 * t6634;
    let t69968 = t5326 * t5390;
    (t69839, t69906, t69910, t69964, t69968)
}
