//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 954/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk954<F: Float>(t4003: F, t6843: F, t10114: F, t10117: F, t10126: F, t10129: F, t14243: F, t14252: F, t1883: F, t213: F, t22009: F, t22329: F, t22333: F, t22337: F, t22353: F, t22362: F, t22366: F, t22370: F, t22374: F, t22381: F, t22964: F, t546: F, t5735: F, t5745: F, t5755: F) -> F {
    let t23037 = t4003 * t6843;
    let t23041 = -F::cast_from(0.58544643236296698113e-1_f64) * t22329 - F::cast_from(0.29272321618148349057e-1_f64) * t22333 - F::cast_from(0.29272321618148349057e-1_f64) * t22337 + F::cast_from(0.39029762157531132076e-1_f64) * t14243 + t10114 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t546 * t22964 - t10117 - F::cast_from(0.16463622957338778996e-1_f64) * t22353 - t10126 - t10129 - F::cast_from(0.39029762157531132076e-1_f64) * t14252 - F::cast_from(0.32927245914677557992e-1_f64) * t22362 + F::cast_from(0.32927245914677557992e-1_f64) * t22366 + F::cast_from(0.16463622957338778996e-1_f64) * t22370 + F::cast_from(0.16463622957338778996e-1_f64) * t22374 + F::cast_from(0.29272321618148349057e-1_f64) * t22381 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t22009 * t1883 + F::cast_from(0.39512695097613069591e1_f64) * t5745 * t5735 * t23037;
    t23041
}
