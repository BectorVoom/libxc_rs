//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1035/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1035(t15292: f64, t15294: f64, t15296: f64, t15298: f64, t15302: f64, t15304: f64, t15306: f64, t15308: f64, t891: f64, t898: f64, t2977: f64, t85: f64) -> (f64, f64) {
    let t15310 = -0.69046666666666666667e1_f64 * t15292 + 0.23015555555555555556e1_f64 * t15294 - 0.26851481481481481482e1_f64 * t15296 - 0.93932222222222222223e0_f64 * t15298 + 0.14671e0_f64 * t15302 - 0.14671e0_f64 * t15304 - 0.17116166666666666667e0_f64 * t15306 - 0.36793333333333333333e0_f64 * t15308;
    let t15312 = t891 * t15310 * t898;
    let t15316 = 1.0_f64 / t2977 / t85;
    (t15312, t15316)
}
