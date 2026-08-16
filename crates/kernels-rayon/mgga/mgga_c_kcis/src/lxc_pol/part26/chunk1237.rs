//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1237/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1237(t27733: f64, t4527: f64, t7671: f64, t1655: f64, t26654: f64, t28311: f64, t28314: f64, t28317: f64, t28320: f64, t28323: f64, t17396: f64, t491: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t95271 = 2.0_f64 * t27733;
    let t97561 = 2.0_f64 * t4527 * t7671;
    let t97601 = t1655 * t26654;
    let t97622 = t28311 / 8.0_f64;
    let t97623 = t28314 / 8.0_f64;
    let t97624 = t28317 / 8.0_f64;
    let t97625 = t28320 / 8.0_f64;
    let t97626 = t28323 / 8.0_f64;
    let t97681 = t17396 * t491;
    (t95271, t97561, t97601, t97622, t97623, t97624, t97625, t97626, t97681)
}
