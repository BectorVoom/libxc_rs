//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1250/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1250(t10680: f64, t11587: f64, t40310: f64, t10673: f64, t11591: f64, t40317: f64, t37459: f64, t37461: f64, t37464: f64, t37468: f64, t37473: f64, t37477: f64, t40426: f64, t40429: f64, t40435: f64, t40451: f64, t40457: f64, t40461: f64, t42953: f64) -> f64 {
    let t43875 = t10680 * t11587 * t40310;
    let t43878 = t10673 * t11591 * t40317;
    let t43883 = t42953 + 0.72042316457491791906e-3_f64 * t43875 - 0.10248087766267884742e-3_f64 * t43878 + t37459 - t37461 - t37464 + t40426 - t40429 + t40435 - 0.43368970657079495312e-4_f64 * t37468 - t37473 - 0.35220688045884876043e-2_f64 * t37477 - 0.30487649791575028314e-3_f64 * t40451 - t40457 + t40461;
    t43883
}
