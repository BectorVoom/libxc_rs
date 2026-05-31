//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1178/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1178<F: Float>(t12589: F, t5749: F, t1131: F, t1150: F, t1173: F, t1181: F, t1532: F, t16529: F, t16533: F, t16537: F, t16542: F, t1753: F, t1879: F, t3282: F, t335: F, t336: F, t367: F, t429: F, t4838: F, t540: F, t5674: F, t6308: F, t6379: F, t6383: F, t839: F, t960: F) -> F {
    let t21361 = t12589 * t5749;
    let t21386 = -F::cast_from(0.17149607247227894789e-2_f64) * t16529 + F::cast_from(0.85748036236139473944e-3_f64) * t16533 + F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t1181 * t1532 * t1753 * t839 + F::cast_from(0.34299214494455789578e-2_f64) * t16537 - F::cast_from(0.68598428988911579156e-2_f64) * t21361 - F::cast_from(0.25724410870841842183e-2_f64) * t16542 - t367 * t336 * t429 * t5674 / F::cast_from(48.0_f64) + t367 * t960 * t540 * t4838 / F::cast_from(24.0_f64) - t1150 * t3282 * t6379 / F::cast_from(8.0_f64) + t1150 * t3282 * t6383 / F::cast_from(4.0_f64) - t335 * t960 * t6308 * t839 / F::cast_from(24.0_f64) - t367 * t960 * t1879 * t1131 / F::cast_from(16.0_f64);
    t21386
}
