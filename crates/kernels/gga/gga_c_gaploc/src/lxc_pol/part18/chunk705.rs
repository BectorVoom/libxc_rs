//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 705/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk705<F: Float>(t2482: F, t6985: F, t2487: F, t1445: F, t6321: F, t4529: F, t874: F, t1328: F, t1555: F, t894: F, t2440: F, t528: F, t1: F, t2293: F, t106: F, t192: F) -> (F, F, F, F, F, F) {
    let t6986 = t6985 * t2482;
    let t6987 = t2487 * t6986;
    let t6989 = t1445 * t6321;
    let t6992 = t4529 * t874;
    let t6993 = t6992 * t1328;
    let t6994 = t1445 * t6993;
    let t6997 = t1555 * t894;
    let t7002 = t528 * t2440;
    let t7005 = t2293 * t1;
    let t7006 = t7005 * t106;
    let t7007 = t7006 * t192;
    (t6987, t6989, t6994, t6997, t7002, t7007)
}
