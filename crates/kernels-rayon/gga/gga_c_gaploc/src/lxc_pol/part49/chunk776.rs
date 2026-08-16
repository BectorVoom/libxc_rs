//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 776/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk776(t13157: f64, t1457: f64, t6060: f64, t1445: f64, t2087: f64, t2558: f64, t3464: f64, t943: f64, t10789: f64, t948: f64, t2508: f64, t10924: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13158 = t1457 * t13157;
    let t13160 = 0.21450293971110256001e1_f64 * t6060 * t13158;
    let t13161 = t1445 * t13157;
    let t13163 = 0.62115540045351614476e2_f64 * t2087 * t13161;
    let t13176 = t3464 * t2558;
    let t13177 = t943 * t13176;
    let t13179 = t10789 * t948;
    let t13180 = t2508 * t13179;
    let t13182 = t10924 * t2558;
    (t13158, t13160, t13161, t13163, t13176, t13177, t13179, t13180, t13182)
}
