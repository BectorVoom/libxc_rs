//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 490/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk490<F: Float>(t2147: F, t2341: F, t157: F, t524: F, t609: F, t2152: F, t159: F, t2331: F, t619: F, t119: F, t1959: F, t1962: F, t2127: F, t2136: F, t2142: F, t2146: F, t2333: F, t2338: F, t557: F, t616: F, t621: F) -> (F, F, F, F) {
    let t2342 = t2147 * t2341;
    let t2346 = t609 * t524 * t157;
    let t2347 = t2152 * t2346;
    let t2351 = t619 * t159 * t2331;
    let t2354 = t1959 - t1962 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t2333 - F::cast_from(0.65854491829355115987e0_f64) * t2127 * t557 - t2136 + t2142 - F::cast_from(0.4336814094102599731e0_f64) * t2338 * t621 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t2342 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t2347 - F::cast_from(0.4336814094102599731e0_f64) * t616 * t2351;
    (t2342, t2347, t2351, t2354)
}
