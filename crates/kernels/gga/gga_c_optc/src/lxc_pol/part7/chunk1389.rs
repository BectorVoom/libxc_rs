//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1389/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1389<F: Float>(t1170: F, t3205: F, t7878: F, t26915: F, t3107: F, t1: F, t123: F, t26894: F, t27011: F, t27027: F, t27202: F, t27204: F, t27209: F, t27210: F, t27533: F, t27553: F, t27616: F, t27619: F, t27621: F, t27623: F, t27630: F, t27644: F, t27651: F, t27667: F, t27670: F, t27671: F, t3106: F, t3186: F, t3192: F, t3217: F, t438: F, t4464: F, t450: F, t458: F, t8969: F, t8973: F, t9049: F, t9058: F, t9169: F, t935: F) -> (F, F) {
    let t27677 = t1170 * t7878 * t3205;
    let t27679 = t26915 * t3107;
    let t27684 = -F::new(0.51515031050600046546e-1) * t27616 - F::new(0.3863627328795003491e-1) * t27619 + F::new(0.1343485452223045261e-1) * t27621 + F::new(0.71903884692229749079e5) * t9169 * t3106 * t27623 + F::new(0.23184437530160156653e8) * t27630 * t450 * t27204 * t935 - F::new(0.27821325036192187983e8) * t27202 * t450 * t27210 * t935 - F::new(0.23229342182245570105e2) * t3192 * t450 * t26915 * t1 * t438 + F::new(0.26631068404529536697e4) * t27644 * t27553 * t8969 - t27651 - F::new(0.13186481011862155443e4) * t3217 * t458 * t26915 * t123 * t438 - F::new(0.61944912485988186948e2) * t8973 * t27011 * t9058 - F::new(0.8790987341241436962e3) * t4464 * t9049 * t27027 - F::new(0.11721316454988582616e4) * t4464 * t27533 * t26894 + F::new(0.3118959061058811624e2) * t27667 - t27670 + F::new(0.81145531355560548285e7) * t27209 * t450 * t27671 * t3107 - F::new(0.779739765264702906e1) * t27677 + F::new(0.69688026546736710315e2) * t3186 * t450 * t27679 * t1;
    (t27679, t27684)
}
