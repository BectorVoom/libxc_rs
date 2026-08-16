//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1285/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1285(t21125: f64, t3883: f64, t26: f64, t11462: f64, t21130: f64, t11408: f64, t11409: f64, t16046: f64, t16052: f64, t16183: f64, t16184: f64, t21186: f64, t21188: f64, t21193: f64, t21196: f64, t21206: f64, t21209: f64, t21212: f64, t21234: f64, t21237: f64, t21240: f64, t21243: f64) -> (f64, f64, f64) {
    let t21245 = t3883 * t21125;
    let t21246 = t26 * t21245;
    let t21248 = t11462 * t21130;
    let t21249 = t26 * t21248;
    let t21267 = -t11408 - 4.0_f64 / 27.0_f64 * t11409 - 8.0_f64 / 27.0_f64 * t16046 + t16183 - t16184 - 4.0_f64 / 9.0_f64 * t16052 + 2.0_f64 / 27.0_f64 * t21186 - 10.0_f64 / 27.0_f64 * t21237 + 4.0_f64 / 3.0_f64 * t21234 + 8.0_f64 / 9.0_f64 * t21240 - 2.0_f64 / 9.0_f64 * t21188 - 2.0_f64 * t21243 - 8.0_f64 / 3.0_f64 * t21206 + t21196 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t21209 + 2.0_f64 / 3.0_f64 * t21212 - t21193 / 3.0_f64;
    (t21246, t21249, t21267)
}
