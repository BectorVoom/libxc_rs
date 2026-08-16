//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1057/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1057(t12694: f64, t12701: f64, t12703: f64, t12706: f64, t12708: f64, t12710: f64, t12714: f64, t12717: f64, t12771: f64, t12774: f64, t12776: f64, t12779: f64, t12782: f64, t12811: f64) -> f64 {
    let t15759 = 0.1125e1_f64 * t12694 + 0.2428125e0_f64 * t12701 - 0.3375e1_f64 * t12703 + 0.12140625e0_f64 * t12706 - 0.5625e0_f64 * t12708 - 0.97125e0_f64 * t12710 - 0.1125e1_f64 * t12714 + 0.97125e0_f64 * t12717 + 0.4046875e-1_f64 * t12771 - 0.485625e0_f64 * t12774 + 0.12140625e0_f64 * t12776 - 0.1875e0_f64 * t12779 + 0.1125e1_f64 * t12782 - 0.4046875e-1_f64 * t12811;
    t15759
}
