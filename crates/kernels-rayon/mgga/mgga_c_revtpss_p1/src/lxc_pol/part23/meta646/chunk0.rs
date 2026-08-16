//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2370/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2370(t10565: f64, t717: f64, t39875: f64, t39894: f64, t9371: f64, t760: f64, t39960: f64, t39963: f64, t2523: f64, t9372: f64, t39909: f64, t738: f64, t745: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40150 = t717 * t10565;
    let t40165 = t39894 * t39875 * t9371;
    let t40167 = 0.12304822629859687989e5_f64 * t760 * t40165;
    let t40169 = t39960 * t39875 * t39963;
    let t40171 = 0.91082604192152556044e5_f64 * t760 * t40169;
    let t40172 = t2523 * t9372;
    let t40182 = t738 * t39909 * t745;
    (t40150, t40165, t40167, t40169, t40171, t40172, t40182)
}
