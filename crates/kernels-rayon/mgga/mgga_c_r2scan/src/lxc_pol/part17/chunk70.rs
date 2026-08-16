//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 70/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk70(t149: f64, t78: f64, t79: f64, t80: f64) -> (f64, f64, f64) {
    let t212 = 0.258925e1_f64 * t149 + t78 + t79 + t80;
    let t215 = 1.0_f64 + 0.29608749977793437516e2_f64 / t212;
    let t216 = f64::ln(t215);
    (t212, t215, t216)
}
