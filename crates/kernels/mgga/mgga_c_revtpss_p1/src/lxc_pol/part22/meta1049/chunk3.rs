//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3689/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3689<F: Float>(t17544: F, t5293: F, t17373: F, t21275: F, t17769: F, t5381: F, t5391: F, t1247: F, t20902: F, t3172: F, t1234: F, t21271: F) -> (F, F, F, F, F, F) {
    let t69773 = t5293 * t17544;
    let t69783 = t21275 * t17373;
    let t69787 = t5381 * t17769;
    let t69789 = t5391 * t17769;
    let t69793 = t1247 * t3172 * t20902;
    let t69795 = t1234 * t21271;
    (t69773, t69783, t69787, t69789, t69793, t69795)
}
