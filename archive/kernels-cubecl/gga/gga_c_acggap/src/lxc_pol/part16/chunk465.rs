//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 465/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk465<F: Float>(t157: F, t406: F, t609: F, t2152: F, t159: F, t2122: F, t619: F, t119: F, t1959: F, t1962: F, t2124: F, t2127: F, t2136: F, t2142: F, t2143: F, t2146: F, t2149: F, t464: F, t616: F, t621: F) -> (F, F, F) {
    let t2154 = t609 * t406 * t157;
    let t2155 = t2152 * t2154;
    let t2159 = t619 * t159 * t2122;
    let t2162 = t1959 - t1962 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t2124 - F::cast_from(0.65854491829355115987e0_f64) * t2127 * t464 - t2136 + t2142 - F::cast_from(0.4336814094102599731e0_f64) * t2143 * t621 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t2149 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t2155 - F::cast_from(0.4336814094102599731e0_f64) * t616 * t2159;
    (t2155, t2159, t2162)
}
