//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1119/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1119(t10710: f64, t10768: f64, t25737: f64, t25499: f64, t37586: f64, t25503: f64, t37658: f64, t11816: f64, t37880: f64, t3308: f64, t6449: f64, t7462: f64) -> (f64, f64, f64, f64, f64) {
    let t39437 = t10768 * t10710 * t25737;
    let t39438 = 0.47609969197673950972e-2_f64 * t39437;
    let t39440 = t37586 * t10710 * t25499;
    let t39443 = t37658 * t10710 * t25503;
    let t39444 = 0.14282990759302185292e-1_f64 * t39443;
    let t39445 = t37880 * t11816;
    let t39446 = 0.47609969197673950972e-2_f64 * t39445;
    let t39448 = t6449 * t3308 * t7462;
    (t39438, t39440, t39444, t39446, t39448)
}
