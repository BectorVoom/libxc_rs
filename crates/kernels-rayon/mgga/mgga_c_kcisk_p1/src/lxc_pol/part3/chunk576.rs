//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 576/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk576(t4825: f64, t4824: f64, t1689: f64, t1692: f64, t172: f64, t342: f64, t569: f64, t142: f64, t673: f64, t1224: f64, t1636: f64, t10: f64, t1797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4826 = 1.0_f64 / t4825;
    let t4827 = t4824 * t4826;
    let t4830 = t1689 * t1692;
    let t4834 = t342 * t172 * t569;
    let t4835 = 0.23744444444444444444e-1_f64 * t4834;
    let t4836 = t142 * t673;
    let t4838 = t1224 * t4836 * t1636;
    let t4840 = t10 * t1797;
    (t4826, t4827, t4830, t4834, t4835, t4836, t4838, t4840)
}
