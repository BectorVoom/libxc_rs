//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 776/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk776<F: Float>(t13157: F, t1457: F, t6060: F, t1445: F, t2087: F, t2558: F, t3464: F, t943: F, t10789: F, t948: F, t2508: F, t10924: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13158 = t1457 * t13157;
    let t13160 = F::cast_from(0.21450293971110256001e1_f64) * t6060 * t13158;
    let t13161 = t1445 * t13157;
    let t13163 = F::cast_from(0.62115540045351614476e2_f64) * t2087 * t13161;
    let t13176 = t3464 * t2558;
    let t13177 = t943 * t13176;
    let t13179 = t10789 * t948;
    let t13180 = t2508 * t13179;
    let t13182 = t10924 * t2558;
    (t13158, t13160, t13161, t13163, t13176, t13177, t13179, t13180, t13182)
}
