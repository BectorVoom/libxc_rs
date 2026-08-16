//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1258/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1258(t35936: f64, t35938: f64, t40330: f64, t40332: f64, t40336: f64, t40340: f64, t40344: f64, t40347: f64, t40350: f64, t40354: f64, t40358: f64, t40361: f64, t40365: f64, t40369: f64, t40371: f64, t40374: f64, t40377: f64, t40381: f64) -> f64 {
    let t42066 = 0.40015750243531754507e-2_f64 * t40330 - 0.80031500487063509015e-2_f64 * t40332 - 0.794625e0_f64 * t35936 - 0.52975e0_f64 * t35938 + 0.13753125e0_f64 * t40336 + 0.183375e0_f64 * t40340 - 0.916875e-1_f64 * t40344 - t40347 / 16.0_f64 - t40350 / 8.0_f64 + 0.183375e0_f64 * t40354 - 0.183375e0_f64 * t40358 - 0.916875e-1_f64 * t40361 - 0.916875e-1_f64 * t40365 - 0.916875e-1_f64 * t40369 + 0.3361875e0_f64 * t40371 - 0.183375e0_f64 * t40374 - 0.916875e-1_f64 * t40377 + 0.4584375e0_f64 * t40381;
    t42066
}
