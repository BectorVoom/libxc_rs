//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 849/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk849(t2769: f64, t2774: f64, t5727: f64, t5736: f64, t5739: f64, t5891: f64, t5895: f64, t5898: f64, t7761: f64, t7807: f64, t7810: f64, t7813: f64, t7817: f64, t951: f64) -> f64 {
    let t8984 = -0.1350520664e0_f64 * t2774 * t2769 - 0.1350520664e0_f64 * t951 * t7761 + t5727 - t5736 - t5739 - t5891 - t5895 - t7807 - 32.0_f64 * t5898 - t7810 - t7813 - 0.20010214504933333333e-2_f64 * t7817;
    t8984
}
