//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 908/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk908(t3128: f64, t879: f64, t4791: f64, t4794: f64, t4798: f64, t4806: f64, t6963: f64, t6966: f64, t8592: f64, t8596: f64, t8600: f64, t8603: f64, t8632: f64) -> f64 {
    let t9797 = t879 * t3128;
    let t9798 = t6963 + 2.0_f64 * t6966 + t9797 - t8592 - t4791 + t4794 + t4798 - t4806 - t8596 - t8600 + t8603 - t8632;
    t9798
}
