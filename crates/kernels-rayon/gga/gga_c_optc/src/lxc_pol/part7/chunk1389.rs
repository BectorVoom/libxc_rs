//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1389/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1389(t1170: f64, t3205: f64, t7878: f64, t26915: f64, t3107: f64, t1: f64, t123: f64, t26894: f64, t27011: f64, t27027: f64, t27202: f64, t27204: f64, t27209: f64, t27210: f64, t27533: f64, t27553: f64, t27616: f64, t27619: f64, t27621: f64, t27623: f64, t27630: f64, t27644: f64, t27651: f64, t27667: f64, t27670: f64, t27671: f64, t3106: f64, t3186: f64, t3192: f64, t3217: f64, t438: f64, t4464: f64, t450: f64, t458: f64, t8969: f64, t8973: f64, t9049: f64, t9058: f64, t9169: f64, t935: f64) -> (f64, f64) {
    let t27677 = t1170 * t7878 * t3205;
    let t27679 = t26915 * t3107;
    let t27684 = -0.51515031050600046546e-1_f64 * t27616 - 0.3863627328795003491e-1_f64 * t27619 + 0.1343485452223045261e-1_f64 * t27621 + 0.71903884692229749079e5_f64 * t9169 * t3106 * t27623 + 0.23184437530160156653e8_f64 * t27630 * t450 * t27204 * t935 - 0.27821325036192187983e8_f64 * t27202 * t450 * t27210 * t935 - 0.23229342182245570105e2_f64 * t3192 * t450 * t26915 * t1 * t438 + 0.26631068404529536697e4_f64 * t27644 * t27553 * t8969 - t27651 - 0.13186481011862155443e4_f64 * t3217 * t458 * t26915 * t123 * t438 - 0.61944912485988186948e2_f64 * t8973 * t27011 * t9058 - 0.8790987341241436962e3_f64 * t4464 * t9049 * t27027 - 0.11721316454988582616e4_f64 * t4464 * t27533 * t26894 + 0.3118959061058811624e2_f64 * t27667 - t27670 + 0.81145531355560548285e7_f64 * t27209 * t450 * t27671 * t3107 - 0.779739765264702906e1_f64 * t27677 + 0.69688026546736710315e2_f64 * t3186 * t450 * t27679 * t1;
    (t27679, t27684)
}
