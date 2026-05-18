//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 471/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk471<F: Float>(t119: F, t1959: F, t1962: F, t2124: F, t2127: F, t2136: F, t2142: F, t2143: F, t2146: F, t2149: F, t2155: F, t2159: F, t464: F, t616: F, t621: F) -> F {
    let t2162 = t1959 - t1962 + F::new(0.65854491829355115987e0) * t119 * t2124 - F::new(0.65854491829355115987e0) * t2127 * t464 - t2136 + t2142 - F::new(0.4336814094102599731e0) * t2143 * t621 + F::new(0.8673628188205199462e0) * t2146 * t2149 + F::new(0.4336814094102599731e0) * t2146 * t2155 - F::new(0.4336814094102599731e0) * t616 * t2159;
    t2162
}
