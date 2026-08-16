//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2257/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2257(t16944: f64, t221: f64, t25154: f64, t16851: f64, t841: f64, t87407: f64, t81903: f64, t87331: f64, t87333: f64, t87336: f64, t87339: f64, t87342: f64, t87348: f64, t87364: f64, t87387: f64, t87402: f64, t92652: f64, t98796: f64, t98798: f64, t98801: f64, t98803: f64, t98808: f64) -> f64 {
    let t98811 = t25154 * t221 * t16944;
    let t98814 = t87407 * t841 * t16851;
    let t98816 = t87331 + t87333 - t87336 + t87339 + t87342 - t92652 - 7.0_f64 / 1152.0_f64 * t98796 + 7.0_f64 / 2304.0_f64 * t98798 - t87348 - 0.20186378047070195427e-3_f64 * t98801 - t98803 / 96.0_f64 - t87364 + 0.10093189023535097713e-3_f64 * t81903 - 0.63250651214153279005e-2_f64 * t87387 - t98808 / 4.0_f64 + t98811 / 8.0_f64 - 0.67826230238155856634e-1_f64 * t98814 - t87402;
    t98816
}
