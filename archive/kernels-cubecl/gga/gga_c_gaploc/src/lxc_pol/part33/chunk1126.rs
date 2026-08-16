//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1126/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1126<F: Float>(t1250: F, t3101: F, t1254: F, t3106: F, t6383: F, t871: F, t23927: F, t9083: F, t29874: F, t9205: F, t123: F, t21004: F, t2326: F, t9074: F) -> (F, F, F, F, F, F) {
    let t29913 = t3101 * t1250;
    let t29915 = t3106 * t1254;
    let t29923 = t6383 * t871;
    let t30003 = F::cast_from(0.47425011059460249332e-2_f64) * t23927 * t9083;
    let t30005 = F::cast_from(0.142275033178380748e-1_f64) * t29874 * t9205;
    let t30009 = F::cast_from(0.142275033178380748e-1_f64) * t9074 * t21004 * t123 * t2326;
    (t29913, t29915, t29923, t30003, t30005, t30009)
}
