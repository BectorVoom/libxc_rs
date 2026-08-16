//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1208/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1208(t39511: f64, t39522: f64, t39506: f64, t39509: f64, t39514: f64, t39517: f64, t39520: f64, t39524: f64, t39526: f64, t39529: f64, t39532: f64, t39535: f64) -> f64 {
    let t41419 = 0.25610080155860322884e0_f64 * t39511;
    let t41423 = 0.46230515946956099004e0_f64 * t39522;
    let t41429 = 0.65854491829355115984e0_f64 * t39506 + 0.32927245914677557992e0_f64 * t39509 + t41419 - 0.54878743191129263322e-1_f64 * t39514 + 0.43663693315433241794e-2_f64 * t39517 + 0.52396431978519890152e-1_f64 * t39520 + t41423 + 0.10401866088065122276e1_f64 * t39524 - 0.17465477326173296718e-1_f64 * t39526 - 0.17465477326173296718e-1_f64 * t39529 + 0.26198215989259945076e-1_f64 * t39532 - 0.26198215989259945077e-1_f64 * t39535;
    t41429
}
