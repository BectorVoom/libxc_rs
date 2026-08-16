//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 487/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk487<F: Float>(t119: F, t1959: F, t1962: F, t2127: F, t2136: F, t2142: F, t2146: F, t2333: F, t2338: F, t2342: F, t2347: F, t2351: F, t557: F, t616: F, t621: F) -> F {
    let t2354 = t1959 - t1962 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t2333 - F::cast_from(0.65854491829355115987e0_f64) * t2127 * t557 - t2136 + t2142 - F::cast_from(0.4336814094102599731e0_f64) * t2338 * t621 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t2342 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t2347 - F::cast_from(0.4336814094102599731e0_f64) * t616 * t2351;
    t2354
}
