//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1116/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1116(t37558: f64, t37560: f64, t41365: f64, t41367: f64, t41368: f64, t43628: f64, t43629: f64, t43631: f64, t43632: f64, t43633: f64, t46300: f64, t46302: f64, t46305: f64, t46307: f64, t46309: f64, t46311: f64) -> f64 {
    let t49151 = 0.35481751119302649979e-2_f64 * t41365 - t41367 + t41368 + t37558 + t43628 + t43629 - t37560 - 0.79828278012425390427e-1_f64 * t46300 + 0.53218852008283593619e-1_f64 * t46302 - t43631 + t43632 + t43633 + 0.17701538806747441785e-2_f64 * t46305 - 0.21241846568096930142e-2_f64 * t46307 + 0.148692925976678511e-1_f64 * t46309 + 0.70806155226989767141e-2_f64 * t46311;
    t49151
}
