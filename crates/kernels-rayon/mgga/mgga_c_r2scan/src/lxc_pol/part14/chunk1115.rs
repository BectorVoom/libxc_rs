//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1115/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1115(t3295: f64, t8018: f64, t1577: f64, t3308: f64, t7438: f64, t10710: f64, t10768: f64, t25737: f64, t25499: f64, t37586: f64, t25503: f64, t37658: f64) -> (f64, f64, f64, f64, f64) {
    let t39431 = t3295 * t8018;
    let t39434 = t1577 * t3308 * t7438;
    let t39437 = t10768 * t10710 * t25737;
    let t39440 = t37586 * t10710 * t25499;
    let t39443 = t37658 * t10710 * t25503;
    (t39431, t39434, t39437, t39440, t39443)
}
