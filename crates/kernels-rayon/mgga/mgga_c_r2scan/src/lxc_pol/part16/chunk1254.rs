//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1254/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1254(t37532: f64, t37542: f64, t37556: f64, t37561: f64, t37564: f64, t37569: f64, t40513: f64, t40515: f64, t40519: f64, t43760: f64, t43764: f64, t43766: f64, t43770: f64, t43774: f64, t43778: f64) -> f64 {
    let t43914 = t37532 + t43760 + t43764 - t43766 - t43770 - t43774 - t37542 - 0.30487649791575028314e-3_f64 * t40513 + 0.30487649791575028314e-3_f64 * t40515 - t40519 + 0.81300399444200075504e-3_f64 * t37556 + t37561 - 0.15243824895787514157e-3_f64 * t37564 - t37569 + t43778;
    t43914
}
