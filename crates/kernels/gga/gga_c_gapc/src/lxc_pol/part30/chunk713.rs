//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 713/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk713<F: Float>(t1022: F, t9209: F, t197: F, t6055: F, t1979: F, t3096: F, t3094: F, t1932: F, t1936: F, t3036: F, t1894: F, t1927: F, t646: F, t3034: F, t3100: F, t659: F) -> (F, F, F, F, F, F) {
    let t9210 = t1022 * t9209;
    let t9213 = t197 * t6055;
    let t9214 = t1022 * t9213;
    let t9216 = t3096 * t1979;
    let t9217 = t3094 * t9216;
    let t9219 = t1932 * t1936;
    let t9220 = t9219 * t3036;
    let t9222 = t1927 * t1894;
    let t9223 = t646 * t9222;
    let t9224 = t3034 * t9223;
    let t9226 = t3100 * t659;
    (t9210, t9214, t9217, t9220, t9224, t9226)
}
