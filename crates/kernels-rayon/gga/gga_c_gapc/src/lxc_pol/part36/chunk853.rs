//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 853/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk853(t10226: f64, t3255: f64, t268: f64, t8350: f64, t2208: f64, t6181: f64, t6201: f64, t3235: f64, t3250: f64, t1004: f64, t2152: f64, t827: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10227 = t10226 * t3255;
    let t10229 = t8350 * t268;
    let t10230 = t10229 * t2208;
    let t10231 = t6181 * t6201;
    let t10232 = t10230 * t10231;
    let t10234 = t3235 * t3250;
    let t10236 = t1004 * t2152;
    let t10237 = t10236 * t827;
    (t10227, t10229, t10230, t10232, t10234, t10237)
}
