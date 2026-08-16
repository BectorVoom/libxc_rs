//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 909/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk909(t4827: f64, t4992: f64, t4996: f64, t5000: f64, t5004: f64, t5008: f64, t6798: f64, t8634: f64, t8636: f64, t8638: f64, t9566: f64, t9569: f64, t9576: f64) -> f64 {
    let t9799 = -t9566 + t8634 + t4992 - t9569 - t8636 - t8638 + t6798 - t4996 + t5000 + t5004 + t5008 - t9576 + t4827;
    t9799
}
