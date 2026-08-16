//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 734/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk734(t1783: f64, t60: f64, t170: f64, t5717: f64, t61: f64, t1376: f64, t697: f64, t1721: f64, t424: f64, t1707: f64, t124: f64, t717: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5902 = t60 * t1783;
    let t5903 = t5902 * t170;
    let t5907 = 0.11407595979765752406e3_f64 * t61 * t5717;
    let t5908 = t1376 * t697;
    let t5910 = t424 * t1721;
    let t5912 = t424 * t1707;
    let t5916 = t124 * t717;
    (t5903, t5907, t5908, t5910, t5912, t5916)
}
