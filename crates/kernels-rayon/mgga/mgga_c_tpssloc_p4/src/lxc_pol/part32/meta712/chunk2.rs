//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2235/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2235(t25038: f64, t25248: f64, t25249: f64, t4119: f64, t28419: f64, t6579: f64, t23035: f64, t23153: f64, t5527: f64, t6637: f64, t22893: f64, t28341: f64, t81640: f64) -> (f64, f64, f64, f64) {
    let t98502 = t25038 * t25248 * t25249 * t4119;
    let t98505 = t6579 * t28419;
    let t98513 = t23035 * t6637 * t23153 * t5527;
    let t98516 = t81640 * t22893 * t28341;
    (t98502, t98505, t98513, t98516)
}
