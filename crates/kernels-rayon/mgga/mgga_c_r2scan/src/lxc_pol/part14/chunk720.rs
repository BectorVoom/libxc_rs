//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 720/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk720(t5673: f64, t188: f64, t5448: f64, t5672: f64, t1893: f64, t644: f64, t1647: f64, t1891: f64, t4831: f64, t4832: f64, t4833: f64, t4834: f64, t4835: f64, t5309: f64, t5312: f64, t5315: f64) -> (f64, f64, f64) {
    let t5674 = 1.0_f64 / t5673;
    let t5675 = t188 * t5674;
    let t5678 = 0.24955700379505800916e5_f64 * t5672 * t5675 * t5448;
    let t5679 = t644 * t1893;
    let t5682 = 0.1551780387578202009e4_f64 * t1891 * t5679 * t1647;
    let t5686 = -0.126595e1_f64 * t5309 + 0.84396666666666666667e0_f64 * t5312 - 0.3938511111111111111e1_f64 * t5315 - t4831 + t4832 - t4833 - t4834 - t4835;
    (t5678, t5682, t5686)
}
