//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 749/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk749<F: Float>(t2385: F, t315: F, t323: F, t157: F, t2217: F, t524: F, t2152: F, t119: F, t2387: F, t310: F, t557: F, t8331: F, t2146: F, t2241: F, t464: F, t8123: F, t8311: F, t8314: F, t8316: F, t8319: F, t8330: F, t8332: F, t8339: F, t9003: F) -> (F, F, F, F, F, F, F) {
    let t9380 = t315 * t2385;
    let t9381 = t9380 * t323;
    let t9385 = t2217 * t524 * t157;
    let t9386 = t2152 * t9385;
    let t9391 = t119 * t2385;
    let t9397 = t310 * t2387;
    let t9399 = t8331 * t557;
    let t9401 = 0.65854491829355115987e0 * t8123 - 0.8673628188205199462e0 * t8311 + 0.8673628188205199462e0 * t8314 - 0.65854491829355115987e0 * t9381 + 0.65854491829355115987e0 * t8319 + 0.4336814094102599731e0 * t2146 * t9386 - 0.65854491829355115987e0 * t8316 * t557 + t8330 - 0.65854491829355115987e0 * t9391 * t464 - 0.65854491829355115987e0 * t8332 + 0.4336814094102599731e0 * t9003 * t2241 - t8339 + 0.65854491829355115987e0 * t9397 - 0.65854491829355115987e0 * t9399;
    (t9380, t9381, t9386, t9391, t9397, t9399, t9401)
}
