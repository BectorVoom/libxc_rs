//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 520/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk520(t121: f64, t3151: f64, t126: f64, t147: f64, t3036: f64, t383: f64, t174: f64, t3037: f64, t386: f64, t387: f64, t1035: f64, t996: f64) -> (f64, f64, f64, f64) {
    let t3206 = t121 * t3151;
    let t3207 = t3206 * t126;
    let t3209 = 455.0_f64 / 1296.0_f64 * t3207 * t147;
    let t3210 = t3036 * t383;
    let t3211 = t174 * t3037;
    let t3213 = t386 * t387 * t3211;
    let t3215 = 0.12862205435420921092e-2_f64 * t3210 * t3213;
    let t3216 = t1035 * t996;
    (t3209, t3213, t3215, t3216)
}
