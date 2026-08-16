//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1148/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1148(t1276: f64, t2938: f64, t3366: f64, t11882: f64, t23498: f64, t263: f64, t2928: f64, t40815: f64, t826: f64, t11880: f64, t11881: f64, t2391: f64, param_eta: f64) -> (f64, f64, f64, f64) {
    let t42508 = t1276 * t3366 * t2938;
    let t42512 = t23498 * param_eta * t11882;
    let t42516 = t40815 * t263 * t2928 * t826;
    let t42519 = t11880 * t11881 * t2391;
    (t42508, t42512, t42516, t42519)
}
