//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2617/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2617(t18643: f64, t40731: f64, t10779: f64, t10786: f64, t14931: f64, t61956: f64, t10811: f64, t18647: f64, t18511: f64, t40864: f64, t10905: f64, t18515: f64) -> (f64, f64, f64, f64, f64) {
    let t62029 = t40731 * t18643;
    let t62033 = t14931 * t10779 * t61956 * t10786;
    let t62045 = t10811 * t18647;
    let t62056 = t40864 * t18511;
    let t62058 = t10905 * t18515;
    (t62029, t62033, t62045, t62056, t62058)
}
