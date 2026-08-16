//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 66/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk66(t183: f64, t190: f64, t149: f64, t65: f64, t66: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t192 = 1.0_f64 * t183 * t190;
    let t194 = 0.3529725e1_f64 * t149 + t65 + t66 + t67;
    let t197 = 1.0_f64 + 0.32163958997385070134e2_f64 / t194;
    let t198 = f64::ln(t197);
    let t200 = t194 * t194;
    let t201 = 1.0_f64 / t200;
    (t192, t194, t197, t198, t200, t201)
}
