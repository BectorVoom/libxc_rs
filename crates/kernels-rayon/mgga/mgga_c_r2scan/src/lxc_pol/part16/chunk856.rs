//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 856/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk856(t3128: f64, t759: f64, t761: f64, t246: f64, t4721: f64, t4901: f64, t4964: f64, t4967: f64, t4972: f64, t7861: f64, t8552: f64, t8555: f64, t8556: f64, t9005: f64) -> f64 {
    let t9040 = t759 * t3128 * t761;
    let t9044 = t8552 - t4901 + t8555 + t7861 + 0.285764e-1_f64 * t9040 - 0.285764e-1_f64 * t246 * t9005 - t4721 + t4964 - t4967 - t8556 - t4972;
    t9044
}
