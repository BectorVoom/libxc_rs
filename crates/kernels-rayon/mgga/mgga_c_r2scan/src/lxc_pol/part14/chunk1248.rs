//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1248/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1248(t322: f64, t41891: f64, t11305: f64, t11319: f64, t11993: f64, t31912: f64, t352: f64, t3556: f64, t38958: f64, t38961: f64, t38971: f64, t38976: f64, t41058: f64, t41065: f64, t855: f64) -> f64 {
    let t332 = 0.25e1_f64 < t322;
    let t42070 = piecewise3(t332, t41891, 0.0_f64);
    let t42098 = -0.105e1_f64 * t855 * t42070 * t352 - 0.126e2_f64 * t3556 * t31912 - 0.63e1_f64 * t3556 * t41065 - 0.252e2_f64 * t11305 * t41058 - 0.567e2_f64 * t11319 * t41058 - 0.189e2_f64 * t38958 * t11993 - 0.945e1_f64 * t11305 * t41065 - 0.189e2_f64 * t11305 * t31912 - 0.2835e2_f64 * t38961 * t41058 - 0.4725e1_f64 * t38971 * t11993 - 0.4725e1_f64 * t11319 * t31912 - 0.23625e1_f64 * t11319 * t41065 - 0.354375e1_f64 * t38976 * t41058;
    t42098
}
