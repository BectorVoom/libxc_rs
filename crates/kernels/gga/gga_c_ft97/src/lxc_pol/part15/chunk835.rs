//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 835/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk835<F: Float>(t1882: F, t20405: F, t20256: F, t20200: F, t103: F, t20113: F, t20260: F, t20276: F, t20179: F, t20428: F, t20220: F, t8392: F, t20205: F, t20281: F, t20196: F, t20193: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t75119 = t1882 * t20405;
    let t75136 = t1882 * t20256;
    let t75138 = t1882 * t20200;
    let t75188 = t103 * t20113;
    let t75227 = t1882 * t20260;
    let t75370 = t1882 * t20276;
    let t75372 = t1882 * t20179;
    let t75391 = t1882 * t20428;
    let t75482 = t8392 * t20220;
    let t75487 = t8392 * t20205;
    let t75489 = t1882 * t20281;
    let t75491 = t1882 * t20196;
    let t75493 = t1882 * t20193;
    (t75119, t75136, t75138, t75188, t75227, t75370, t75372, t75391, t75482, t75487, t75489, t75491, t75493)
}
