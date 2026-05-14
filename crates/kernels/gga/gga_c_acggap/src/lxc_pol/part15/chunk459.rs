//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 459/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk459<F: Float>(t2138: F, t2230: F, t463: F, t633: F, t2147: F, t157: F, t2152: F, t406: F, t159: F, t2217: F, t619: F, t119: F, t2143: F, t2146: F, t2175: F, t2178: F, t2219: F, t2222: F, t2228: F, t464: F, t616: F, t639: F) -> (F, F, F, F, F) {
    let t2232 = 0.8673628188205199462e0 * t2138 * t2230;
    let t2235 = t633 * t463;
    let t2236 = t2147 * t2235;
    let t2241 = t2152 * t633 * t406 * t157;
    let t2245 = t619 * t159 * t2217;
    let t2248 = t2175 - t2178 + 0.65854491829355115987e0 * t119 * t2219 - 0.65854491829355115987e0 * t2222 * t464 - t2228 + t2232 - 0.4336814094102599731e0 * t2143 * t639 + 0.8673628188205199462e0 * t2146 * t2236 + 0.4336814094102599731e0 * t2146 * t2241 - 0.4336814094102599731e0 * t616 * t2245;
    (t2232, t2236, t2241, t2245, t2248)
}
