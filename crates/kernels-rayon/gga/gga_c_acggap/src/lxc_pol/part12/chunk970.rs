//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 970/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk970(t694: f64, t8052: f64, t2236: f64, t30005: f64, t3054: f64, t633: f64, t865: f64, t2245: f64, t7924: f64, t7987: f64, t8100: f64, t1264: f64, t2131: f64, t2147: f64, t2225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32313 = t694 * t8052;
    let t32315 = t30005 * t2236;
    let t32324 = 0.39512695097613069591e1_f64 * t3054 * t633 * t865;
    let t32329 = t7924 * t2245;
    let t32331 = t7987 * t8100;
    let t32335 = t2131 * t2147 * t2225 * t1264;
    (t32313, t32315, t32324, t32329, t32331, t32335)
}
