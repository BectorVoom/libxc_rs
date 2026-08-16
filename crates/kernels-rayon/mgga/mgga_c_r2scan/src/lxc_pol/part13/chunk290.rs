//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 290/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk290(t44: f64, t51: f64, t538: f64, t921: f64, t529: f64, t889: f64, t99: f64, t101: f64, t893: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t927 = t538 * t921;
    let t928 = t529 * t927;
    let t933 = piecewise3(t45, 0.0_f64, 5.0_f64 / 3.0_f64 * t99 * t889);
    let t936 = piecewise3(t52, 0.0_f64, 5.0_f64 / 3.0_f64 * t101 * t893);
    let t938 = t933 / 2.0_f64 + t936 / 2.0_f64;
    (t927, t928, t938)
}
