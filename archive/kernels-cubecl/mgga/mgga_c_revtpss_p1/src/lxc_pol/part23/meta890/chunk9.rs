//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2840/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2840<F: Float>(t23289: F, t2741: F, t2661: F, t2662: F, t6035: F, t61625: F, t124: F, t40782: F, t50943: F, t50955: F, t50978: F, t62021: F, t62029: F, t62033: F, t62045: F, t62056: F, t62058: F, t62069: F, t62072: F, t62089: F, t62095: F, t62105: F, t76421: F, t799: F, t800: F) -> F {
    let t76793 = t2741 * t23289;
    let t76797 = t2661 * t2662 * t61625 * t6035;
    let t76800 = -F::cast_from(0.54885603034028154483e-3_f64) * t50943 + F::cast_from(0.85748036236139473944e-4_f64) * t62021 + t50955 - F::cast_from(0.32524801797942610064e-2_f64) * t62029 - F::cast_from(0.30492001685571196935e-3_f64) * t62033 - F::cast_from(0.24009450146119052704e-1_f64) * t62045 - t799 * t800 * t124 * t76421 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4.0_f64) * t62056 - F::cast_from(7.0_f64) / F::cast_from(8.0_f64) * t62058 + F::cast_from(0.76230004213927992336e-5_f64) * t62069 - F::cast_from(0.15246000842785598467e-4_f64) * t62072 + t50978 + F::cast_from(0.15117061203111996148e0_f64) * t40782 + F::cast_from(35.0_f64) / F::cast_from(24.0_f64) * t62089 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t62095 + F::cast_from(0.10003937560882938627e-2_f64) * t76793 - F::cast_from(0.85748036236139473942e-4_f64) * t76797 - F::cast_from(0.22869001264178397701e-3_f64) * t62105;
    t76800
}
