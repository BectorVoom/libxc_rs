//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1141/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1141(t10907: f64, t2207: f64, t3606: f64, t10760: f64, t24814: f64, t6085: f64, t25569: f64, t6093: f64, t22868: f64, t24831: f64, t11717: f64, t19883: f64) -> (f64, f64, f64, f64, f64) {
    let t39686 = t2207 * t10907 * t3606;
    let t39689 = t6085 * t10760 * t24814;
    let t39692 = t6093 * t10760 * t25569;
    let t39695 = t22868 * t10760 * t24831;
    let t39697 = t19883 * t11717;
    (t39686, t39689, t39692, t39695, t39697)
}
