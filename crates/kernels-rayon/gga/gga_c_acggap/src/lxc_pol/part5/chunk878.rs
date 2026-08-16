//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 878/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk878(t1098: f64, t3228: f64, t1108: f64, t3244: f64, t1086: f64, t1113: f64, t1032: f64, t3348: f64, t3328: f64, t377: f64, t947: f64, t1036: f64, t1095: f64, t1131: f64, t398: f64, t864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12838 = t3228 * t1098;
    let t12840 = t3244 * t1108;
    let t12842 = t3228 * t1086;
    let t12844 = t3244 * t1113;
    let t12848 = t1032 * t3348;
    let t12854 = t377 * t3328;
    let t12855 = t12854 * t947;
    let t12862 = t1036 * t398 * t1095 * t1131 * t864;
    (t12838, t12840, t12842, t12844, t12848, t12854, t12855, t12862)
}
