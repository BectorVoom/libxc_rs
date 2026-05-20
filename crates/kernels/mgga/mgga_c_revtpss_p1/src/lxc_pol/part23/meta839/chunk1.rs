//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2713/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2713<F: Float>(t20879: F, t3172: F, t3711: F, t1260: F, t20850: F, t11262: F, t3600: F, t6630: F, t17225: F, t5391: F, t21183: F, t20875: F) -> (F, F, F, F, F, F) {
    let t69899 = t3711 * t3172 * t20879;
    let t69906 = t20850 * t1260;
    let t69910 = t3600 * t11262 * t6630;
    let t69916 = t5391 * t17225;
    let t69936 = t3711 * t3172 * t21183;
    let t69939 = t3711 * t3172 * t20875;
    (t69899, t69906, t69910, t69916, t69936, t69939)
}
