//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 858/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk858(t1068: f64, t3213: f64, t3215: f64, t390: f64, t10521: f64, t10528: f64, t10607: f64, t10625: f64, t10627: f64, t10635: f64, t1070: f64, t10711: f64, t10729: f64, t10733: f64, t10849: f64, t10851: f64, t11087: f64, t193: f64, t336: f64) -> f64 {
    let t11091 = t3213 * t1068;
    let t11094 = 1.0_f64 / t3215 / t390;
    let t11098 = t1070 * t11087 * t193 * t336 + 2.0_f64 * t11091 * t11094 * t193 * t336 - t10521 + t10528 - t10607 - t10625 - t10627 - t10635 - t10711 - t10729 + t10733 + t10849 + t10851;
    t11098
}
