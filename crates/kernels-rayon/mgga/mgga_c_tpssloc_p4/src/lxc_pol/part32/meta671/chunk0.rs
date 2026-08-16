//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2104/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2104(t24649: f64, t27710: f64, t23508: f64, t8026: f64, t27628: f64, t7324: f64, t7331: f64, t15730: f64, t7339: f64, t24661: f64, t27491: f64, t24668: f64, t27497: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95323 = t27710 * t24649;
    let t95326 = t8026 * t23508;
    let t95332 = t7324 * t27628;
    let t95334 = 0.20186378047070195428e-3_f64 * t95332 * t7331;
    let t95335 = t7339 * t15730;
    let t95340 = t24661 * t27491;
    let t95346 = t24668 * t27497;
    (t95323, t95326, t95332, t95334, t95335, t95340, t95346)
}
