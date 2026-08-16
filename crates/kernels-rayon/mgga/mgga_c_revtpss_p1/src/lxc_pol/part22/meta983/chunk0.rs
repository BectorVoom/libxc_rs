//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3333/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3333(t15390: f64, t15421: f64, t11294: f64, t19318: f64, t11528: f64, t19321: f64, t19324: f64, t41883: f64, t11289: f64, t6142: f64, t19128: f64, t2869: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63218 = 0.64327917994770140268e2_f64 * t15421 * t15390;
    let t63220 = 12.0_f64 * t11294 * t19318;
    let t63222 = 8.0_f64 * t11528 * t19321;
    let t63224 = 0.19298375398431042081e3_f64 * t41883 * t19324;
    let t63226 = 1.0_f64 * t11289 * t6142;
    let t63228 = 2.0_f64 * t2869 * t19128;
    (t63218, t63220, t63222, t63224, t63226, t63228)
}
