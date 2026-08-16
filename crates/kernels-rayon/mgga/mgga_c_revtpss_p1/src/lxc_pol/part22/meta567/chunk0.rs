//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2410/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2410(t4343: f64, t4542: f64, t2404: f64, t5966: f64, t14613: f64, t162: f64, t4403: f64, t14312: f64, t5940: f64, t705: f64, t707: f64, t10605: f64, t6002: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18253 = t4542 * t4343;
    let t18256 = t2404 * t5966;
    let t18259 = t14613 * t162;
    let t18261 = 24.0_f64 * t18259 * t4403;
    let t18262 = 2.0_f64 * t14312;
    let t18263 = t705 * t5940;
    let t18265 = 4.0_f64 * t18263 * t707;
    let t18267 = 12.0_f64 * t10605 * t6002;
    (t18253, t18256, t18259, t18261, t18262, t18263, t18265, t18267)
}
