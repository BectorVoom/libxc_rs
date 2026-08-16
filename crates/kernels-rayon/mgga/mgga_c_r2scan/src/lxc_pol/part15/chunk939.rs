//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 939/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk939(t2135: f64, t3308: f64, t10776: f64, t20: f64, t2214: f64, t3293: f64) -> (f64, f64, f64, f64) {
    let t10777 = t3308 * t2135;
    let t10778 = t10776 * t10777;
    let t10780 = t2214 * t20;
    let t10781 = t3293 * t10780;
    (t10777, t10778, t10780, t10781)
}
