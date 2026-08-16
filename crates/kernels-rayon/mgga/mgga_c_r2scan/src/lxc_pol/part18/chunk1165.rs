//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1165/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1165(t11486: f64, t11629: f64, t3262: f64, t10663: f64, t12567: f64, t11515: f64, t12570: f64, t481: f64, t10610: f64, t3263: f64, t3579: f64, t39203: f64) -> (f64, f64, f64, f64, f64) {
    let t42929 = 15.0_f64 / 8.0_f64 * t3262 * t11629 * t11486;
    let t42931 = t12567 * t10663 / 4.0_f64;
    let t42933 = t12567 * t11515 / 4.0_f64;
    let t42934 = t12570 * t481;
    let t42937 = 3.0_f64 / 2.0_f64 * t10610 * t3263 * t42934;
    let t42939 = t3579 * t39203 / 2.0_f64;
    (t42929, t42931, t42933, t42937, t42939)
}
