//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1136/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1136(t26186: f64, t3308: f64, t6528: f64, t10810: f64, t2583: f64, t574: f64, t10788: f64, t3613: f64, t5095: f64, t20825: f64, t3610: f64, t10776: f64, t8002: f64) -> (f64, f64, f64, f64, f64) {
    let t39789 = t6528 * t3308 * t26186;
    let t39792 = t574 * t10810 * t2583;
    let t39795 = t5095 * t3613 * t10788;
    let t39801 = t20825 * t3610;
    let t39804 = t10776 * t3308 * t8002;
    (t39789, t39792, t39795, t39801, t39804)
}
