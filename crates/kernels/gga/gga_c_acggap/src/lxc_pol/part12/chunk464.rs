//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 464/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk464<F: Float>(t119: F, t2143: F, t2146: F, t2175: F, t2178: F, t2219: F, t2222: F, t2228: F, t2232: F, t2236: F, t2241: F, t2245: F, t464: F, t616: F, t639: F) -> (F,) {
    let t2248 = t2175 - t2178 + 0.65854491829355115987e0 * t119 * t2219 - 0.65854491829355115987e0 * t2222 * t464 - t2228 + t2232 - 0.4336814094102599731e0 * t2143 * t639 + 0.8673628188205199462e0 * t2146 * t2236 + 0.4336814094102599731e0 * t2146 * t2241 - 0.4336814094102599731e0 * t616 * t2245;
    (t2248,)
}
