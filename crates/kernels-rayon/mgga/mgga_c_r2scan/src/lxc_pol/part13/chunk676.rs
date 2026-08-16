//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 676/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk676(t1376: f64, t2: f64, t464: f64, t1520: f64, t1531: f64, t386: f64, t518: f64, t85: f64, t462: f64, t1510: f64, t406: f64, t1512: f64, t410: f64) -> (f64, f64, f64, f64, f64) {
    let t5011 = t1376 * t2;
    let t5012 = t5011 * t464;
    let t5015 = t1520 * t1531;
    let t5018 = t386 * t518 * t85;
    let t5019 = t462 * t5018;
    let t5020 = 0.56968947174242584612e-3_f64 * t5019;
    let t5021 = t406 * t1510;
    let t5027 = t410 * t1512;
    (t5012, t5015, t5020, t5021, t5027)
}
