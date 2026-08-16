//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 581/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk581(t678: f64, t7939: f64, t7210: f64, t7213: f64, t7245: f64, t7270: f64, t7280: f64, t7289: f64, t4616: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7940 = t7939 * t678;
    let t8019 = 0.162600798888400151e-2_f64 * t7210;
    let t8020 = 0.162600798888400151e-2_f64 * t7213;
    let t8026 = 0.39726959900411316772e-4_f64 * t7245;
    let t8036 = 0.36366215538993788974e-1_f64 * t7270;
    let t8038 = 0.1454648621559751559e0_f64 * t7280;
    let t8040 = 0.10909864661698136692e0_f64 * t7289;
    let t8041 = t4616 * t698;
    (t7940, t8019, t8020, t8026, t8036, t8038, t8040, t8041)
}
