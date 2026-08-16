//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2057/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2057(t87910: f64, t1519: f64, t212: f64, t23171: f64, t6554: f64, t25040: f64, t82074: f64, t87712: f64, t25193: f64, t81591: f64, t10143: f64, t7540: f64) -> (f64, f64, f64, f64, f64) {
    let t87911 = 0.82246703342411321824e-2_f64 * t87910;
    let t87915 = t23171 * t212 * t1519 * t6554;
    let t87927 = t87712 * t82074 * t25040;
    let t87931 = t81591 * t25193;
    let t87932 = 0.76763589786250567036e-1_f64 * t87931;
    let t87975 = t7540 * t10143;
    (t87911, t87915, t87927, t87932, t87975)
}
