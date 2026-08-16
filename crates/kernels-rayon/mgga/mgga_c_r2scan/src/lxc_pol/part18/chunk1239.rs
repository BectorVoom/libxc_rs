//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1239/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1239(t3232: f64, t4176: f64, t3270: f64, t3269: f64, t10663: f64, t12422: f64, t12384: f64, t37271: f64, t11626: f64, t40713: f64, t11540: f64, t40276: f64) -> (f64, f64, f64, f64, f64) {
    let t43775 = t4176 * t3232;
    let t43776 = t3270 * t43775;
    let t43778 = t3269 * t43776 / 4.0_f64;
    let t43780 = t12422 * t10663 / 4.0_f64;
    let t43782 = 5.0_f64 / 8.0_f64 * t37271 * t12384;
    let t43783 = t40713 * t11626;
    let t43785 = t40276 * t11540 / 2.0_f64;
    (t43778, t43780, t43782, t43783, t43785)
}
