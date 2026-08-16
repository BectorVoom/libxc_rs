//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 946/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk946(t10846: f64, t3320: f64, t481: f64, t2207: f64, t3319: f64, t2161: f64, t2164: f64, t505: f64, t502: f64, t57: f64, t512: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10847 = 0.46574606203128791246e-1_f64 * t10846;
    let t10848 = t3320 * t481;
    let t10850 = t2207 * t3319 * t10848;
    let t10851 = 0.13972381860938637374e0_f64 * t10850;
    let t10853 = t2161 * t505 * t2164;
    let t10854 = 0.81312004494856525156e-4_f64 * t10853;
    let t10855 = t502 * t57;
    let t10856 = t512 * t10855;
    (t10847, t10848, t10850, t10851, t10854, t10855, t10856)
}
