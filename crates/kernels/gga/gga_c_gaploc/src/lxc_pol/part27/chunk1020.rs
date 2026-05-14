//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1020/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1020<F: Float>(t18089: F, t18096: F, t27846: F, t4066: F, t92: F, t27842: F, t4082: F, t4085: F, t1250: F, t3101: F, t1254: F, t3106: F, t6383: F, t871: F, t23927: F, t9083: F) -> (F, F, F, F, F, F) {
    let t29908 = t18096 * t4066 * t27846 * t18089 * t92;
    let t29911 = t4082 * t27842 * t4085;
    let t29913 = t3101 * t1250;
    let t29915 = t3106 * t1254;
    let t29923 = t6383 * t871;
    let t30003 = 0.47425011059460249332e-2 * t23927 * t9083;
    (t29908, t29911, t29913, t29915, t29923, t30003)
}
