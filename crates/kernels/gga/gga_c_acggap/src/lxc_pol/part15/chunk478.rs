//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 478/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk478<F: Float>(t119: F, t2146: F, t2175: F, t2178: F, t2222: F, t2228: F, t2232: F, t2338: F, t2387: F, t2395: F, t2400: F, t2404: F, t557: F, t616: F, t639: F) -> (F,) {
    let t2407 = t2175 - t2178 + 0.65854491829355115987e0 * t119 * t2387 - 0.65854491829355115987e0 * t2222 * t557 - t2228 + t2232 - 0.4336814094102599731e0 * t2338 * t639 + 0.8673628188205199462e0 * t2146 * t2395 + 0.4336814094102599731e0 * t2146 * t2400 - 0.4336814094102599731e0 * t616 * t2404;
    (t2407,)
}
